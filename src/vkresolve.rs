//! Vulkan Function Resolver

#![cfg(feature = "Implements")]
#![allow(non_snake_case)]

use crate::vk::*;
use crate::VkResultBox;
use cfg_if::cfg_if;
#[cfg(feature = "DynamicLoaded")]
use libloading::*;

#[cfg(feature = "VK_KHR_xlib_surface")]
use x11::xlib::{Display, VisualID};
#[cfg(feature = "VK_KHR_xcb_surface")]
use xcb::ffi::xcb_connection_t;

use libc::*;

// Replacement Formula(RegEx)
// * NoReturn API: pub fn (\w+)\((([^\)]|[\r\n])*)\)\s*; => WrapAPI!($1 = $1($2));
// * Return API: pub fn (\w+)\((([^\)]|[\r\n])*)\)\s*->\s*([^;\s]*)\s*; => WrapAPI!($1 = $1($2) -> $4);

use std::cell::RefCell;
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering;
thread_local!(static STATIC_RESOLVER_INITIALIZED: RefCell<bool> = RefCell::new(false));
#[cfg(not(feature = "CustomResolver"))]
static STATIC_RESOLVER: AtomicPtr<Resolver> = AtomicPtr::new(std::ptr::null_mut());
#[cfg(feature = "CustomResolver")]
static STATIC_RESOLVER: AtomicPtr<ResolverInterface> = AtomicPtr::new(0 as *mut _);

#[cfg(feature = "CustomResolver")]
pub fn set_custom_resolver(resv: Box<ResolverInterface>) {
    STATIC_RESOLVER_INITIALIZED.with(|f| {
        if !*f.borrow() {
            let _ =
                STATIC_RESOLVER.compare_exchange(0 as *mut _, Box::into_raw(resv), Ordering::SeqCst, Ordering::Relaxed);
        }
    });
}

cfg_if! {
    if #[cfg(feature = "DynamicLoaded")] {
        pub struct DefaultGlobalResolver;
        impl ResolverInterface2 for DefaultGlobalResolver {
            unsafe fn load_symbol_unconstrainted<T: FromPtr>(&self, name: &[u8]) -> T {
                Resolver::get().load_symbol_unconstrainted(name)
            }
        }
    }
}

pub unsafe trait FromPtr {
    unsafe fn from_ptr(p: *const c_void) -> Self;
}
pub unsafe trait PFN: FromPtr {
    const NAME_NUL: &'static [u8];
}

pub struct ResolvedFnCell<F: PFN, R>(R, std::sync::OnceLock<F>);
impl<F: PFN, R: ResolverInterface2> ResolvedFnCell<F, R> {
    pub const fn new(resolver: R) -> Self {
        Self(resolver, std::sync::OnceLock::new())
    }

    pub fn resolve(&self) -> &F {
        self.1
            .get_or_init(|| unsafe { self.0.load_symbol_unconstrainted::<F>(F::NAME_NUL) })
    }
}

macro_rules! WrapAPI2 {
    { #[org = $org_fn: ident] $v: vis fn $name: ident($($arg_name: ident: $arg_type: ty),* $(,)?) -> VkResult; $($rest: tt)* } => {
        cfg_if! {
            if #[cfg(feature = "DynamicLoaded")] {
                $v unsafe fn $name($($arg_name: $arg_type),*) -> VkResultBox {
                    #[repr(transparent)]
                    pub struct FT(extern "system" fn($($arg_type),*) -> VkResult);
                    unsafe impl FromPtr for FT {
                        unsafe fn from_ptr(p: *const c_void) -> Self {
                            core::mem::transmute(p)
                        }
                    }
                    unsafe impl PFN for FT {
                        const NAME_NUL: &'static [u8] = concat!(stringify!($org_name), "\0").as_bytes();
                    }

                    static F: ResolvedFnCell<FT, DefaultGlobalResolver> = ResolvedFnCell::new(DefaultGlobalResolver);

                    log::trace!(target: "br-vkapi-call", stringify!($org_name));

                    VkResultBox(F.resolve().0($($arg_name),*))
                }
            } else {
                $v unsafe fn $name($($arg_name: $arg_type),*) -> VkResultBox {
                    log::trace!(target: "br-vkapi-call", stringify!($org_name));

                    VkResultBox($org_fn($($arg_name),*))
                }
            }
        }

        WrapAPI2!($($rest)*);
    };
    { #[org = $org_fn: ident] $v: vis fn $name: ident ($($arg_name: ident: $arg_type: ty),* $(,)?) -> $rt: ty; $($rest: tt)* } => {
        cfg_if! {
            if #[cfg(feature = "DynamicLoaded")] {
                $v unsafe fn $name($($arg_name: $arg_type),*) -> $rt {
                    #[repr(transparent)]
                    pub struct FT(extern "system" fn($($arg_type),*) -> $rt);
                    unsafe impl FromPtr for FT {
                        unsafe fn from_ptr(p: *const c_void) -> Self {
                            core::mem::transmute(p)
                        }
                    }
                    unsafe impl PFN for FT {
                        const NAME_NUL: &'static [u8] = concat!(stringify!($org_name), "\0").as_bytes();
                    }

                    static F: ResolvedFnCell<FT, DefaultGlobalResolver> = ResolvedFnCell::new(DefaultGlobalResolver);

                    log::trace!(target: "br-vkapi-call", stringify!($org_name));

                    F.resolve().0($($arg_name),*)
                }
            } else {
                $v unsafe fn $name($($arg_name: $arg_type),*) -> $rt {
                    log::trace!(target: "br-vkapi-call", stringify!($org_name));

                    $org_fn($($arg_name),*)
                }
            }
        }

        WrapAPI2!($($rest)*);
    };
    { #[org = $org_fn: ident] $v: vis fn $name: ident ($($arg_name: ident: $arg_type: ty),* $(,)?); $($rest: tt)* } => {
        cfg_if! {
            if #[cfg(feature = "DynamicLoaded")] {
                $v unsafe fn $name($($arg_name: $arg_type),*) {
                    #[repr(transparent)]
                    pub struct FT(extern "system" fn($($arg_type),*));
                    unsafe impl FromPtr for FT {
                        unsafe fn from_ptr(p: *const c_void) -> Self {
                            core::mem::transmute(p)
                        }
                    }
                    unsafe impl PFN for FT {
                        const NAME_NUL: &'static [u8] = concat!(stringify!($org_name), "\0").as_bytes();
                    }

                    static F: ResolvedFnCell<FT, DefaultGlobalResolver> = ResolvedFnCell::new(DefaultGlobalResolver);

                    log::trace!(target: "br-vkapi-call", stringify!($org_name));

                    F.resolve().0($($arg_name),*)
                }
            } else {
                $v unsafe fn $name($($arg_name: $arg_type),*) {
                    log::trace!(target: "br-vkapi-call", stringify!($org_name));

                    $org_fn($($arg_name),*)
                }
            }
        }

        WrapAPI2!($($rest)*);
    };
    {} => {}
}

WrapAPI2!(
    #[org = vkCreateInstance]
    pub fn create_instance(
        create_info: *const VkInstanceCreateInfo,
        allocator: *const VkAllocationCallbacks,
        instance_out: *mut VkInstance,
    ) -> VkResult;
    #[org = vkDestroyInstance]
    pub fn destroy_instance(instance: VkInstance, allocator: *const VkAllocationCallbacks);

    #[org = vkEnumeratePhysicalDevices]
    pub fn enumerate_physical_devices(
        instance: VkInstance,
        physical_devices_count_out: *mut u32,
        physical_devices_out: *mut VkPhysicalDevice,
    ) -> VkResult;

    #[org = vkGetPhysicalDeviceFeatures]
    pub fn get_physical_device_features(physical_device: VkPhysicalDevice, features_out: *mut VkPhysicalDeviceFeatures);
    #[org = vkGetPhysicalDeviceFormatProperties]
    pub fn get_physical_device_format_properties(
        physical_device: VkPhysicalDevice,
        format: VkFormat,
        format_properties_out: *mut VkFormatProperties,
    );
    #[org = vkGetPhysicalDeviceImageFormatProperties]
    pub fn get_physical_device_image_format_properties(
        physical_device: VkPhysicalDevice,
        format: VkFormat,
        image_type: VkImageType,
        tiling: VkImageTiling,
        usage: VkImageUsageFlags,
        flags: VkImageCreateFlags,
        image_format_properties_out: *mut VkImageFormatProperties,
    ) -> VkResult;
    #[org = vkGetPhysicalDeviceProperties]
    pub fn get_physical_device_properties(
        physical_device: VkPhysicalDevice,
        properties_out: *mut VkPhysicalDeviceProperties,
    );
    #[org = vkGetPhysicalDeviceQueueFamilyProperties]
    pub fn get_physical_device_queue_family_properties(
        physical_device: VkPhysicalDevice,
        queue_family_properties_count_out: *mut u32,
        queue_family_properties_out: *mut VkQueueFamilyProperties,
    );
    #[org = vkGetPhysicalDeviceMemoryProperties]
    pub fn get_physical_device_memory_properties(
        physical_device: VkPhysicalDevice,
        memory_properties_out: *mut VkPhysicalDeviceMemoryProperties,
    );

    #[org = vkGetInstanceProcAddr]
    pub fn get_instance_proc_addr(instance: VkInstance, name: *const c_char) -> Option<PFN_vkVoidFunction>;
    #[org = vkGetDeviceProcAddr]
    pub fn get_device_proc_addr(device: VkDevice, name: *const c_char) -> Option<PFN_vkVoidFunction>;

    #[org = vkCreateDevice]
    pub fn create_device(
        physical_device: VkPhysicalDevice,
        create_info: *const VkDeviceCreateInfo,
        allocator: *const VkAllocationCallbacks,
        device_out: *mut VkDevice,
    ) -> VkResult;
    #[org = vkDestroyDevice]
    pub fn destroy_device(device: VkDevice, allocator: *const VkAllocationCallbacks);

    #[org = vkEnumerateInstanceExtensionProperties]
    pub fn enumerate_instance_extension_properties(
        layer_name: *const c_char,
        property_count_out: *mut u32,
        properties_out: *mut VkExtensionProperties,
    ) -> VkResult;
    #[org = vkEnumerateDeviceExtensionProperties]
    pub fn enumerate_device_extension_properties(
        physical_device: VkPhysicalDevice,
        layer_name: *const c_char,
        property_count_out: *mut u32,
        properties_out: *mut VkExtensionProperties,
    ) -> VkResult;

    #[org = vkEnumerateInstanceLayerProperties]
    pub fn enumerate_instance_layer_properties(
        property_count_out: *mut u32,
        properties_out: *mut VkLayerProperties,
    ) -> VkResult;
    #[org = vkEnumerateDeviceLayerProperties]
    pub fn enumerate_device_layer_properties(
        physical_device: VkPhysicalDevice,
        property_count_out: *mut u32,
        properties_out: *mut VkLayerProperties,
    ) -> VkResult;

    #[org = vkGetDeviceQueue]
    pub fn get_device_queue(device: VkDevice, queue_family_index: u32, queue_index: u32, queue_out: *mut VkQueue);
    #[org = vkQueueSubmit]
    pub fn queue_submit(queue: VkQueue, submit_count: u32, submits: *const VkSubmitInfo, fence: VkFence) -> VkResult;
    #[org = vkQueueWaitIdle]
    pub fn queue_wait_idle(queue: VkQueue) -> VkResult;
    #[org = vkDeviceWaitIdle]
    pub fn device_wait_idle(device: VkDevice) -> VkResult;

    #[org = vkAllocateMemory]
    pub fn allocate_memory(
        device: VkDevice,
        allocate_info: *const VkMemoryAllocateInfo,
        allocator: *const VkAllocationCallbacks,
        memory_out: *mut VkDeviceMemory,
    ) -> VkResultBox;
    #[org = vkFreeMemory]
    pub fn free_memory(device: VkDevice, memory: VkDeviceMemory, allocator: *const VkAllocationCallbacks);

    #[org = vkMapMemory]
    pub fn map_memory(
        device: VkDevice,
        memory: VkDeviceMemory,
        offset: VkDeviceSize,
        size: VkDeviceSize,
        flags: VkMemoryMapFlags,
        data_ptr_out: *mut *mut c_void,
    ) -> VkResult;
    #[org = vkUnmapMemory]
    pub fn unmap_memory(device: VkDevice, memory: VkDeviceMemory);

    #[org = vkFlushMappedMemoryRanges]
    pub fn flush_mapped_memory_ranges(
        device: VkDevice,
        memory_range_count: u32,
        memory_ranges: *const VkMappedMemoryRange,
    ) -> VkResult;
    #[org = vkInvalidateMappedMemoryRanges]
    pub fn invalidate_mapped_memory_ranges(
        device: VkDevice,
        memory_range_count: u32,
        memory_ranges: *const VkMappedMemoryRange,
    ) -> VkResult;

    #[org = vkGetDeviceMemoryCommitment]
    pub fn get_device_memory_commitment(
        device: VkDevice,
        memory: VkDeviceMemory,
        committed_memory_bytes_out: *mut VkDeviceSize,
    );

    #[org = vkBindBufferMemory]
    pub fn bind_buffer_memory(
        device: VkDevice,
        buffer: VkBuffer,
        memory: VkDeviceMemory,
        memory_offset: VkDeviceSize,
    ) -> VkResult;
    #[org = vkBindImageMemory]
    pub fn bind_image_memory(
        device: VkDevice,
        image: VkImage,
        memory: VkDeviceMemory,
        memoryOffset: VkDeviceSize,
    ) -> VkResult;

    #[org = vkGetBufferMemoryRequirements]
    pub fn get_buffer_memory_requirements(
        device: VkDevice,
        buffer: VkBuffer,
        memory_requirements_out: *mut VkMemoryRequirements,
    );
    #[org = vkGetImageMemoryRequirements]
    pub fn get_image_memory_requirements(
        device: VkDevice,
        image: VkImage,
        memory_requirements_out: *mut VkMemoryRequirements,
    );

    #[org = vkGetImageSparseMemoryRequirements]
    pub fn get_image_sparse_memory_requirements(
        device: VkDevice,
        image: VkImage,
        sparse_memory_requirement_count_out: *mut u32,
        sparse_memory_requirements_out: *mut VkSparseImageMemoryRequirements,
    );
    #[org = vkGetPhysicalDeviceSparseImageFormatProperties]
    pub fn get_physical_device_sparse_image_format_properties(
        physical_device: VkPhysicalDevice,
        format: VkFormat,
        r#type: VkImageType,
        samples: VkSampleCountFlags,
        usage: VkImageUsageFlags,
        tiling: VkImageTiling,
        property_count_out: *mut u32,
        properites_out: *mut VkSparseImageFormatProperties,
    );
    #[org = vkQueueBindSparse]
    pub fn queue_bind_sparse(
        queue: VkQueue,
        bind_info_count: u32,
        bind_info: *const VkBindSparseInfo,
        fence: VkFence,
    ) -> VkResult;

    #[org = vkCreateFence]
    pub fn create_fence(
        device: VkDevice,
        create_info: *const VkFenceCreateInfo,
        allocator: *const VkAllocationCallbacks,
        fence_out: *mut VkFence,
    ) -> VkResult;
    #[org = vkDestroyFence]
    pub fn destroy_fence(device: VkDevice, fence: VkFence, allocator: *const VkAllocationCallbacks);

    #[org = vkResetFences]
    pub fn reset_fences(device: VkDevice, fence_count: u32, fences: *const VkFence) -> VkResult;
    #[org = vkGetFenceStatus]
    pub fn get_fence_status(device: VkDevice, fence: VkFence) -> VkResult;
    #[org = vkWaitForFences]
    pub fn wait_for_fences(
        device: VkDevice,
        fence_count: u32,
        fences: *const VkFence,
        wait_all: VkBool32,
        timeout: u64,
    ) -> VkResult;

    #[org = vkCreateSemaphore]
    pub fn create_semaphore(
        device: VkDevice,
        create_info: *const VkSemaphoreCreateInfo,
        allocator: *const VkAllocationCallbacks,
        semaphore_out: *mut VkSemaphore,
    ) -> VkResult;
    #[org = vkDestroySemaphore]
    pub fn destroy_semaphore(device: VkDevice, semaphore: VkSemaphore, allocator: *const VkAllocationCallbacks);

    #[org = vkCreateEvent]
    pub fn create_event(
        device: VkDevice,
        create_info: *const VkEventCreateInfo,
        allocator: *const VkAllocationCallbacks,
        event_out: *mut VkEvent,
    ) -> VkResult;
    #[org = vkDestroyEvent]
    pub fn destroy_event(device: VkDevice, event: VkEvent, allocator: *const VkAllocationCallbacks);

    #[org = vkGetEventStatus]
    pub fn get_event_status(device: VkDevice, event: VkEvent) -> VkResult;
    #[org = vkSetEvent]
    pub fn set_event(device: VkDevice, event: VkEvent) -> VkResult;
    #[org = vkResetEvent]
    pub fn reset_event(device: VkDevice, event: VkEvent) -> VkResult;

    #[org = vkCreateQueryPool]
    pub fn create_query_pool(
        device: VkDevice,
        create_info: *const VkQueryPoolCreateInfo,
        allocator: *const VkAllocationCallbacks,
        query_pool_out: *mut VkQueryPool,
    ) -> VkResult;
    #[org = vkDestroyQueryPool]
    pub fn destroy_query_pool(device: VkDevice, query_pool: VkQueryPool, allocator: *const VkAllocationCallbacks);

    #[org = vkGetQueryPoolResults]
    pub fn get_query_pool_results(
        device: VkDevice,
        query_pool: VkQueryPool,
        first_query: u32,
        query_count: u32,
        data_size: size_t,
        data_out: *mut c_void,
        stride: VkDeviceSize,
        flags: VkQueryResultFlags,
    ) -> VkResult;

    #[org = vkCreateBuffer]
    pub fn create_buffer(
        device: VkDevice,
        create_info: *const VkBufferCreateInfo,
        allocator: *const VkAllocationCallbacks,
        buffer_out: *mut VkBuffer,
    ) -> VkResult;
    #[org = vkDestroyBuffer]
    pub fn destroy_buffer(device: VkDevice, buffer: VkBuffer, allocator: *const VkAllocationCallbacks);

    #[org = vkCreateBufferView]
    pub fn create_buffer_view(
        device: VkDevice,
        create_info: *const VkBufferViewCreateInfo,
        allocator: *const VkAllocationCallbacks,
        view_out: *mut VkBufferView,
    ) -> VkResult;
    #[org = vkDestroyBufferView]
    pub fn destroy_buffer_view(device: VkDevice, buffer_view: VkBufferView, allocator: *const VkAllocationCallbacks);

    #[org = vkCreateImage]
    pub fn create_image(
        device: VkDevice,
        create_info: *const VkImageCreateInfo,
        allocator: *const VkAllocationCallbacks,
        image_out: *mut VkImage,
    ) -> VkResult;
    #[org = vkDestroyImage]
    pub fn destroy_image(device: VkDevice, image: VkImage, allocator: *const VkAllocationCallbacks);

    #[org = vkGetImageSubresourceLayout]
    pub fn get_image_subresource_layout(
        device: VkDevice,
        image: VkImage,
        subresource: *const VkImageSubresource,
        layout_out: *mut VkSubresourceLayout,
    );

    #[org = vkCreateImageView]
    pub fn create_image_view(
        device: VkDevice,
        create_info: *const VkImageViewCreateInfo,
        allocator: *const VkAllocationCallbacks,
        view_out: *mut VkImageView,
    ) -> VkResult;
    #[org = vkDestroyImageView]
    pub fn destroy_image_view(device: VkDevice, image_view: VkImageView, allocator: *const VkAllocationCallbacks);

    #[org = vkCreateShaderModule]
    pub fn create_shader_module(
        device: VkDevice,
        create_info: *const VkShaderModuleCreateInfo,
        allocator: *const VkAllocationCallbacks,
        shader_module_out: *mut VkShaderModule,
    ) -> VkResult;
    #[org = vkDestroyShaderModule]
    pub fn destroy_shader_module(
        device: VkDevice,
        shader_module: VkShaderModule,
        allocator: *const VkAllocationCallbacks,
    );

    #[org = vkCreatePipelineCache]
    pub fn create_pipeline_cache(
        device: VkDevice,
        create_info: *const VkPipelineCacheCreateInfo,
        allocator: *const VkAllocationCallbacks,
        pipeline_cache_out: *mut VkPipelineCache,
    ) -> VkResult;
    #[org = vkDestroyPipelineCache]
    pub fn destroy_pipeline_cache(
        device: VkDevice,
        pipeline_cache: VkPipelineCache,
        allocator: *const VkAllocationCallbacks,
    );

    #[org = vkGetPipelineCacheData]
    pub fn get_pipeline_cache_data(
        device: VkDevice,
        pipeline_cache: VkPipelineCache,
        data_size_out: *mut size_t,
        data_out: *mut c_void,
    ) -> VkResult;
    #[org = vkMergePipelienCaches]
    pub fn merge_pipeline_caches(
        device: VkDevice,
        dst_cache: VkPipelineCache,
        src_cache_count: u32,
        src_caches: *const VkPipelineCache,
    ) -> VkResult;

    #[org = vkCreateGraphicsPipelines]
    pub fn create_graphics_pipelines(
        device: VkDevice,
        pipeline_cache: VkPipelineCache,
        create_info_count: u32,
        create_infos: *const VkGraphicsPipelineCreateInfo,
        allocator: *const VkAllocationCallbacks,
        pipelines_out: *mut VkPipeline,
    ) -> VkResult;
    #[org = vkCreateComputePipelines]
    pub fn create_compute_pipelines(
        device: VkDevice,
        pipeline_cache: VkPipelineCache,
        create_info_count: u32,
        create_infos: *const VkComputePipelineCreateInfo,
        allocator: *const VkAllocationCallbacks,
        pipelines_out: *mut VkPipeline,
    ) -> VkResult;
    #[org = vkDestroyPipeline]
    pub fn destroy_pipeline(device: VkDevice, pipeline: VkPipeline, allocator: *const VkAllocationCallbacks);

    #[org = vkCreatePipelineLayout]
    pub fn create_pipeline_layout(
        device: VkDevice,
        create_info: *const VkPipelineLayoutCreateInfo,
        allocator: *const VkAllocationCallbacks,
        pipeline_layout_out: *mut VkPipelineLayout,
    ) -> VkResult;
    #[org = vkDestroyPipelineLayout]
    pub fn destroy_pipeline_layout(
        device: VkDevice,
        pipeline_layout: VkPipelineLayout,
        allocator: *const VkAllocationCallbacks,
    );

    #[org = vkCreateSampler]
    pub fn create_sampler(
        device: VkDevice,
        create_info: *const VkSamplerCreateInfo,
        allocator: *const VkAllocationCallbacks,
        sampler_out: *mut VkSampler,
    ) -> VkResult;
    #[org = vkDestroySampler]
    pub fn destroy_sampler(device: VkDevice, sampler: VkSampler, allocator: *const VkAllocationCallbacks);

    #[org = vkCreateDescriptorSetLayout]
    pub fn create_descriptor_set_layout(
        device: VkDevice,
        create_info: *const VkDescriptorSetLayoutCreateInfo,
        allocator: *const VkAllocationCallbacks,
        set_layout_out: *mut VkDescriptorSetLayout,
    ) -> VkResult;
    #[org = vkDestroyDescriptorSetLayout]
    pub fn destroy_descriptor_set_layout(
        device: VkDevice,
        descriptor_set_layout: VkDescriptorSetLayout,
        allocator: *const VkAllocationCallbacks,
    );

    #[org = vkCreateDescriptorPool]
    pub fn create_descriptor_pool(
        device: VkDevice,
        create_info: *const VkDescriptorPoolCreateInfo,
        allocator: *const VkAllocationCallbacks,
        descriptor_pool_out: *mut VkDescriptorPool,
    ) -> VkResult;
    #[org = vkDestroyDescriptorPool]
    pub fn destroy_descriptor_pool(
        device: VkDevice,
        descriptor_pool: VkDescriptorPool,
        allocator: *const VkAllocationCallbacks,
    );

    #[org = vkResetDescriptorPool]
    pub fn reset_descriptor_pool(
        device: VkDevice,
        descriptor_pool: VkDescriptorPool,
        flags: VkDescriptorPoolResetFlags,
    ) -> VkResult;
    #[org = vkAllocateDescriptorSets]
    pub fn allocate_descriptor_sets(
        device: VkDevice,
        allocate_info: *const VkDescriptorSetAllocateInfo,
        descriptor_sets_out: *mut VkDescriptorSet,
    ) -> VkResult;
    #[org = vkFreeDescriptorSets]
    pub fn free_descriptor_sets(
        device: VkDevice,
        descriptor_pool: VkDescriptorPool,
        descriptor_set_count: u32,
        descriptor_sets: *const VkDescriptorSet,
    ) -> VkResult;
    #[org = vkUpdateDescriptorSets]
    pub fn update_descriptor_sets(
        device: VkDevice,
        descriptor_write_count: u32,
        descriptor_writes: *const VkWriteDescriptorSet,
        descriptor_copy_count: u32,
        descriptor_copies: *const VkCopyDescriptorSet,
    );

    #[org = vkCreateFramebuffer]
    pub fn create_framebuffer(
        device: VkDevice,
        create_info: *const VkFramebufferCreateInfo,
        allocator: *const VkAllocationCallbacks,
        framebuffer_out: *mut VkFramebuffer,
    ) -> VkResult;
    #[org = vkDestroyFramebuffer]
    pub fn destroy_framebuffer(device: VkDevice, framebuffer: VkFramebuffer, allocator: *const VkAllocationCallbacks);

    #[org = vkCreateRenderPass]
    pub fn create_render_pass(
        device: VkDevice,
        create_info: *const VkRenderPassCreateInfo,
        allocator: *const VkAllocationCallbacks,
        render_pass_out: *mut VkRenderPass,
    ) -> VkResult;
    #[org = vkDestroyRenderPass]
    pub fn destroy_render_pass(device: VkDevice, render_pass: VkRenderPass, allocator: *const VkAllocationCallbacks);

    #[org = vkGetRenderAreaGranularity]
    pub fn get_render_area_granularity(device: VkDevice, render_pass: VkRenderPass, granularity_out: *mut VkExtent2D);

    #[org = vkCreateCommandPool]
    pub fn create_command_pool(
        device: VkDevice,
        create_info: *const VkCommandPoolCreateInfo,
        allocator: *const VkAllocationCallbacks,
        command_pool_out: *mut VkCommandPool,
    ) -> VkResult;
    #[org = vkDestroyCommandPool]
    pub fn destroy_command_pool(device: VkDevice, command_pool: VkCommandPool, allocator: *const VkAllocationCallbacks);

    #[org = vkResetCommandPool]
    pub fn reset_command_pool(
        device: VkDevice,
        command_pool: VkCommandPool,
        flags: VkCommandPoolResetFlags,
    ) -> VkResult;
    #[org = vkAllocateCommandBuffers]
    pub fn allocate_command_buffers(
        device: VkDevice,
        allocate_info: *const VkCommandBufferAllocateInfo,
        command_buffers_out: *mut VkCommandBuffer,
    ) -> VkResult;
    #[org = vkFreeCommandBuffers]
    pub fn free_command_buffers(
        device: VkDevice,
        command_pool: VkCommandPool,
        command_buffer_count: u32,
        command_buffers: *const VkCommandBuffer,
    );

    #[org = vkBeginCommandBuffer]
    pub fn begin_command_buffer(
        command_buffer: VkCommandBuffer,
        begin_info: *const VkCommandBufferBeginInfo,
    ) -> VkResult;
    #[org = vkEndCommandBuffer]
    pub fn end_command_buffer(command_buffer: VkCommandBuffer) -> VkResult;
    #[org = vkResetCommandBuffer]
    pub fn reset_command_buffer(command_buffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResultBox;
);

pub trait ResolverInterface2 {
    unsafe fn load_symbol_unconstrainted<T: FromPtr>(&self, name: &[u8]) -> T;
}
pub trait ResolverInterface {
    unsafe fn cmd_bind_pipeline(
        &self,
        commandBuffer: VkCommandBuffer,
        pipelineBindPoint: VkPipelineBindPoint,
        pipeline: VkPipeline,
    );
    unsafe fn cmd_set_viewport(
        &self,
        commandBuffer: VkCommandBuffer,
        firstViewport: u32,
        viewportCount: u32,
        pViewports: *const VkViewport,
    );
    unsafe fn cmd_set_scissor(
        &self,
        commandBuffer: VkCommandBuffer,
        firstScissor: u32,
        scissorCount: u32,
        pScissors: *const VkRect2D,
    );
    unsafe fn cmd_set_line_width(&self, commandBuffer: VkCommandBuffer, lineWidth: c_float);
    unsafe fn cmd_set_depth_bias(
        &self,
        commandBuffer: VkCommandBuffer,
        depthBiasConstantFactor: c_float,
        depthBiasClamp: c_float,
        depthBiasSlopeFactor: c_float,
    );
    unsafe fn cmd_set_blend_constants(&self, commandBuffer: VkCommandBuffer, blendConstants: *const c_float);
    unsafe fn cmd_set_depth_bounds(
        &self,
        commandBuffer: VkCommandBuffer,
        minDepthBounds: c_float,
        maxDepthBounds: c_float,
    );
    unsafe fn cmd_set_stencil_compare_mask(
        &self,
        commandBuffer: VkCommandBuffer,
        faceMask: VkStencilFaceFlags,
        compareMask: u32,
    );
    unsafe fn cmd_set_stencil_write_mask(
        &self,
        commandBuffer: VkCommandBuffer,
        faceMask: VkStencilFaceFlags,
        writeMask: u32,
    );
    unsafe fn cmd_set_stencil_reference(
        &self,
        commandBuffer: VkCommandBuffer,
        faceMask: VkStencilFaceFlags,
        reference: u32,
    );
    unsafe fn cmd_bind_descriptor_sets(
        &self,
        commandBuffer: VkCommandBuffer,
        pipelineBindPoint: VkPipelineBindPoint,
        layout: VkPipelineLayout,
        firstSet: u32,
        descriptorSetCount: u32,
        pDescriptorSets: *const VkDescriptorSet,
        dynamicOffsetCount: u32,
        pDynamicOffsets: *const u32,
    );
    unsafe fn cmd_bind_index_buffer(
        &self,
        commandBuffer: VkCommandBuffer,
        buffer: VkBuffer,
        offset: VkDeviceSize,
        indexType: VkIndexType,
    );
    unsafe fn cmd_bind_vertex_buffers(
        &self,
        commandBuffer: VkCommandBuffer,
        firstBinding: u32,
        bindingCount: u32,
        pBuffers: *const VkBuffer,
        pOffsets: *const VkDeviceSize,
    );
    unsafe fn cmd_draw(
        &self,
        commandBuffer: VkCommandBuffer,
        vertexCount: u32,
        instanceCount: u32,
        firstVertex: u32,
        firstInstance: u32,
    );
    unsafe fn cmd_draw_indexed(
        &self,
        commandBuffer: VkCommandBuffer,
        indexCount: u32,
        instanceCount: u32,
        firstIndex: u32,
        vertexOffset: i32,
        firstInstance: u32,
    );
    unsafe fn cmd_draw_indirect(
        &self,
        commandBuffer: VkCommandBuffer,
        buffer: VkBuffer,
        offset: VkDeviceSize,
        drawCount: u32,
        stride: u32,
    );
    unsafe fn cmd_draw_indexed_indirect(
        &self,
        commandBuffer: VkCommandBuffer,
        buffer: VkBuffer,
        offset: VkDeviceSize,
        drawCount: u32,
        stride: u32,
    );
    unsafe fn cmd_dispatch(&self, commandBuffer: VkCommandBuffer, groupCountX: u32, groupCountY: u32, groupCountZ: u32);
    unsafe fn cmd_dispatch_indirect(&self, commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize);
    unsafe fn cmd_copy_buffer(
        &self,
        commandBuffer: VkCommandBuffer,
        srcBuffer: VkBuffer,
        dstBuffer: VkBuffer,
        regionCount: u32,
        pRegions: *const VkBufferCopy,
    );
    unsafe fn cmd_copy_image(
        &self,
        commandBuffer: VkCommandBuffer,
        srcImage: VkImage,
        srcImageLayout: VkImageLayout,
        dstImage: VkImage,
        dstImageLayout: VkImageLayout,
        regionCount: u32,
        pRegions: *const VkImageCopy,
    );
    unsafe fn cmd_blit_image(
        &self,
        commandBuffer: VkCommandBuffer,
        srcImage: VkImage,
        srcImageLayout: VkImageLayout,
        dstImage: VkImage,
        dstImageLayout: VkImageLayout,
        regionCount: u32,
        pRegions: *const VkImageBlit,
        filter: VkFilter,
    );
    unsafe fn cmd_copy_buffer_to_image(
        &self,
        commandBuffer: VkCommandBuffer,
        srcBuffer: VkBuffer,
        dstImage: VkImage,
        dstImageLayout: VkImageLayout,
        regionCount: u32,
        pRegions: *const VkBufferImageCopy,
    );
    unsafe fn cmd_copy_image_to_buffer(
        &self,
        commandBuffer: VkCommandBuffer,
        srcImage: VkImage,
        srcImageLayout: VkImageLayout,
        dstBuffer: VkBuffer,
        regionCount: u32,
        pRegions: *const VkBufferImageCopy,
    );
    unsafe fn cmd_update_buffer(
        &self,
        commandBuffer: VkCommandBuffer,
        dstBuffer: VkBuffer,
        dstOffset: VkDeviceSize,
        dataSize: VkDeviceSize,
        pData: *const c_void,
    );
    unsafe fn cmd_fill_buffer(
        &self,
        ommandBuffer: VkCommandBuffer,
        dstBuffer: VkBuffer,
        dstOffset: VkDeviceSize,
        size: VkDeviceSize,
        data: u32,
    );
    unsafe fn cmd_clear_color_image(
        &self,
        commandBuffer: VkCommandBuffer,
        image: VkImage,
        imageLayout: VkImageLayout,
        pColor: *const VkClearColorValue,
        rangeCount: u32,
        pRanges: *const VkImageSubresourceRange,
    );
    unsafe fn cmd_clear_depth_stencil_image(
        &self,
        commandBuffer: VkCommandBuffer,
        image: VkImage,
        imageLayout: VkImageLayout,
        pDepthStencil: *const VkClearDepthStencilValue,
        rangeCount: u32,
        pRanges: *const VkImageSubresourceRange,
    );
    unsafe fn cmd_clear_attachments(
        &self,
        commandBuffer: VkCommandBuffer,
        attachmentCount: u32,
        pAttachments: *const VkClearAttachment,
        rectCount: u32,
        pRects: *const VkClearRect,
    );
    unsafe fn cmd_resolve_image(
        &self,
        commandBuffer: VkCommandBuffer,
        srcImage: VkImage,
        srcImageLayout: VkImageLayout,
        dstImage: VkImage,
        dstImageLayout: VkImageLayout,
        regionCount: u32,
        pRegions: *const VkImageResolve,
    );
    unsafe fn cmd_set_event(&self, commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);
    unsafe fn cmd_reset_event(&self, commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);
    unsafe fn cmd_wait_events(
        &self,
        commandBuffer: VkCommandBuffer,
        eventCount: u32,
        pEvents: *const VkEvent,
        srcStageMask: VkPipelineStageFlags,
        dstStageMask: VkPipelineStageFlags,
        memoryBarrierCount: u32,
        pMemoryBarriers: *const VkMemoryBarrier,
        bufferMemoryBarrierCount: u32,
        pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
        imageMemoryBarrierCount: u32,
        pImageMemoryBariers: *const VkImageMemoryBarrier,
    );
    unsafe fn cmd_pipeline_barrier(
        &self,
        commandBuffer: VkCommandBuffer,
        srcStageMask: VkPipelineStageFlags,
        dstStageMask: VkPipelineStageFlags,
        dependencyFlags: VkDependencyFlags,
        memoryBarrierCount: u32,
        pMemoryBarriers: *const VkMemoryBarrier,
        bufferMemoryBarrierCount: u32,
        pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
        imageMemoryBarrierCount: u32,
        pImageMemoryBarriers: *const VkImageMemoryBarrier,
    );
    unsafe fn cmd_begin_query(
        &self,
        commandBuffer: VkCommandBuffer,
        queryPool: VkQueryPool,
        query: u32,
        flags: VkQueryControlFlags,
    );
    unsafe fn cmd_end_query(&self, commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32);
    unsafe fn cmd_reset_query_pool(
        &self,
        commandBuffer: VkCommandBuffer,
        queryPool: VkQueryPool,
        firstQuery: u32,
        queryCount: u32,
    );
    unsafe fn cmd_write_timestamp(
        &self,
        commandBuffer: VkCommandBuffer,
        pipelineStage: VkPipelineStageFlags,
        queryPool: VkQueryPool,
        query: u32,
    );
    unsafe fn cmd_copy_query_pool_results(
        &self,
        commandBuffer: VkCommandBuffer,
        queryPool: VkQueryPool,
        firstQuery: u32,
        queryCount: u32,
        dstBuffer: VkBuffer,
        dstOffset: VkDeviceSize,
        stride: VkDeviceSize,
        flags: VkQueryResultFlags,
    );
    unsafe fn cmd_push_constants(
        &self,
        commandBuffer: VkCommandBuffer,
        layout: VkPipelineLayout,
        stageFlags: VkShaderStageFlags,
        offset: u32,
        size: u32,
        pValues: *const c_void,
    );
    unsafe fn cmd_begin_render_pass(
        &self,
        commandBuffer: VkCommandBuffer,
        pRenderPassBegin: *const VkRenderPassBeginInfo,
        contents: VkSubpassContents,
    );
    unsafe fn cmd_next_subpass(&self, commandBuffer: VkCommandBuffer, contents: VkSubpassContents);
    unsafe fn cmd_end_render_pass(&self, commandBuffer: VkCommandBuffer);
    unsafe fn cmd_execute_commands(
        &self,
        commandBuffer: VkCommandBuffer,
        commandBufferCount: u32,
        pCommandBuffers: *const VkCommandBuffer,
    );
    #[cfg(feature = "VK_KHR_push_descriptor")]
    unsafe fn cmd_push_descriptor_set_khr(
        &self,
        commandBuffer: VkCommandBuffer,
        pipelineBindPoint: VkPipelineBindPoint,
        layout: VkPipelineLayout,
        set: u32,
        descriptorWriteCount: u32,
        pDescriptorWrites: *const VkWriteDescriptorSet,
    );
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    #[cfg(feature = "VK_KHR_push_descriptor")]
    unsafe fn push_descriptor_set_with_template_khr(
        &self,
        commandBuffer: VkCommandBuffer,
        descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR,
        layout: VkPipelineLayout,
        set: u32,
        pData: *const c_void,
    );
    #[cfg(feature = "VK_EXT_debug_marker")]
    unsafe fn cmd_debug_marker_begin_ext(
        &self,
        commandBuffer: VkCommandBuffer,
        pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT,
    );
    #[cfg(feature = "VK_EXT_debug_marker")]
    unsafe fn cmd_debug_marker_end_ext(&self, commandBuffer: VkCommandBuffer);
    #[cfg(feature = "VK_EXT_debug_marker")]
    unsafe fn cmd_debug_marker_insert_ext(
        &self,
        commandBuffer: VkCommandBuffer,
        pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT,
    );
    #[cfg(feature = "VK_AMD_draw_indirect_count")]
    unsafe fn cmd_draw_indirect_count_amd(
        &self,
        commandBuffer: VkCommandBuffer,
        buffer: VkBuffer,
        offset: VkDeviceSize,
        countBuffer: VkBuffer,
        countBufferOffset: VkDeviceSize,
        maxDrawCount: u32,
        stride: u32,
    );
    #[cfg(feature = "VK_AMD_draw_indirect_count")]
    unsafe fn cmd_draw_indexed_indirect_count_amd(
        &self,
        commandBuffer: VkCommandBuffer,
        buffer: VkBuffer,
        offset: VkDeviceSize,
        countBuffer: VkBuffer,
        countBufferOffset: VkDeviceSize,
        maxDrawCount: u32,
        stride: u32,
    );
    #[cfg(feature = "VK_KHX_device_group")]
    unsafe fn cmd_set_device_mask_khx(&self, commandBuffer: VkCommandBuffer, deviceMask: u32);
    #[cfg(feature = "VK_KHX_device_group")]
    unsafe fn cmd_dispatch_base_khx(
        &self,
        commandBuffer: VkCommandBuffer,
        baseGroupX: u32,
        baseGroupY: u32,
        baseGroupZ: u32,
        groupCountX: u32,
        groupCountY: u32,
        groupCountZ: u32,
    );
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    unsafe fn cmd_process_commands_nvx(
        &self,
        commandBuffer: VkCommandBuffer,
        pProcessCommandsInfo: *const VkCmdProcessCommandsInfoNVX,
    );
    #[cfg(feature = "VK_NVX_device_generated_commands")]
    unsafe fn cmd_reserve_space_for_commands_nvx(
        &self,
        commandBuffer: VkCommandBuffer,
        pReserveSpaceInfo: *const VkCmdReserveSpaceForCommandsInfoNVX,
    );
    #[cfg(feature = "VK_NV_clip_space_w_scaling")]
    unsafe fn cmd_set_viewport_w_scaling_nv(
        &self,
        commandBuffer: VkCommandBuffer,
        firstViewport: u32,
        viewportCount: u32,
        pViewportWScalings: *const VkViewportWScalingNV,
    );
    #[cfg(feature = "VK_EXT_discard_rectangles")]
    unsafe fn cmd_discard_rectangle_ext(
        &self,
        commandBuffer: VkCommandBuffer,
        firstDiscardRectangle: u32,
        discardRectangleCount: u32,
        pDiscardRectangles: *const VkRect2D,
    );

    #[cfg(feature = "VK_KHR_surface")]
    unsafe fn destroy_surface_khr(
        &self,
        instance: VkInstance,
        surface: VkSurfaceKHR,
        pAllocator: *const VkAllocationCallbacks,
    );
    #[cfg(feature = "VK_KHR_surface")]
    unsafe fn get_physical_device_surface_support_khr(
        &self,
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        surface: VkSurfaceKHR,
        pSupported: *mut VkBool32,
    ) -> VkResultBox;
    #[cfg(feature = "VK_KHR_surface")]
    unsafe fn get_physical_device_surface_capabilities_khr(
        &self,
        physicalDevice: VkPhysicalDevice,
        surface: VkSurfaceKHR,
        pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR,
    ) -> VkResultBox;
    #[cfg(feature = "VK_KHR_surface")]
    unsafe fn get_physical_device_surface_formats_khr(
        &self,
        physicalDevice: VkPhysicalDevice,
        surface: VkSurfaceKHR,
        pSurfaceFormatCount: *mut u32,
        pSurfaceFormats: *mut VkSurfaceFormatKHR,
    ) -> VkResultBox;
    #[cfg(feature = "VK_KHR_surface")]
    unsafe fn get_physical_device_surface_present_modes_khr(
        &self,
        physicalDevice: VkPhysicalDevice,
        surface: VkSurfaceKHR,
        pPresentModeCount: *mut u32,
        pPresentModes: *mut VkPresentModeKHR,
    ) -> VkResultBox;
    #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
    unsafe fn get_physical_device_surface_capabilities2_khr(
        &self,
        physicalDevice: VkPhysicalDevice,
        surface_info: *const VkPhysicalDeviceSurfaceInfo2KHR,
        surface_capabilities: *mut VkSurfaceCapabilities2KHR,
    ) -> VkResultBox;

    #[cfg(feature = "VK_KHR_swapchain")]
    unsafe fn create_swapchain_khr(
        &self,
        device: VkDevice,
        pCreateInfo: *const VkSwapchainCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSwapchain: *mut VkSwapchainKHR,
    ) -> VkResultBox;
    #[cfg(feature = "VK_KHR_swapchain")]
    unsafe fn destroy_swapchain_khr(
        &self,
        device: VkDevice,
        swapchain: VkSwapchainKHR,
        pAllocator: *const VkAllocationCallbacks,
    );
    #[cfg(feature = "VK_KHR_swapchain")]
    unsafe fn get_swapchain_images_khr(
        &self,
        device: VkDevice,
        swapchain: VkSwapchainKHR,
        pSwapchainImageCount: *mut u32,
        pSwapchainImages: *mut VkImage,
    ) -> VkResultBox;
    #[cfg(feature = "VK_KHR_swapchain")]
    unsafe fn acquire_next_image_khr(
        &self,
        device: VkDevice,
        swapchain: VkSwapchainKHR,
        timeout: u64,
        semaphore: VkSemaphore,
        fence: VkFence,
        pImageIndex: *mut u32,
    ) -> VkResultBox;
    #[cfg(feature = "VK_KHR_swapchain")]
    unsafe fn queue_present_khr(&self, queue: VkQueue, pPresentInfo: *const VkPresentInfoKHR) -> VkResultBox;

    #[cfg(feature = "VK_KHR_xlib_surface")]
    unsafe fn create_xlib_surface_khr(
        &self,
        instance: VkInstance,
        pCreateInfo: *const VkXlibSurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResultBox;
    #[cfg(feature = "VK_KHR_xlib_surface")]
    unsafe fn get_physical_device_xlib_presentation_support_khr(
        &self,
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        dpy: *mut Display,
        visualID: VisualID,
    ) -> VkBool32;

    #[cfg(feature = "VK_KHR_xcb_surface")]
    unsafe fn create_xcb_surface_khr(
        &self,
        instance: VkInstance,
        pCreateInfo: *const VkXcbSurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResultBox;
    #[cfg(feature = "VK_KHR_xcb_surface")]
    unsafe fn get_physical_device_xcb_presentation_support_khr(
        &self,
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        connection: *mut xcb_connection_t,
        visual_id: xcb::x::Visualid,
    ) -> VkBool32;

    #[cfg(feature = "VK_KHR_wayland_surface")]
    unsafe fn create_wayland_surface_khr(
        &self,
        instance: VkInstance,
        pCreateInfo: *const VkWaylandSurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResultBox;
    #[cfg(feature = "VK_KHR_wayland_surface")]
    unsafe fn get_physical_device_wayland_presentation_support_khr(
        &self,
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        display: *mut wayland_client::sys::wl_display,
    ) -> VkBool32;

    #[cfg(feature = "VK_KHR_android_surface")]
    unsafe fn create_android_surface_khr(
        &self,
        instance: VkInstance,
        pCreateInfo: *const VkAndroidSurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResultBox;

    #[cfg(feature = "VK_KHR_win32_surface")]
    unsafe fn create_win32_surface_khr(
        &self,
        instance: VkInstance,
        pCreateInfo: *const VkWin32SurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResultBox;
    #[cfg(feature = "VK_KHR_win32_surface")]
    unsafe fn get_physical_device_win32_presentation_support_khr(
        &self,
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
    ) -> VkBool32;

    #[cfg(feature = "VK_MVK_macos_surface")]
    unsafe fn create_macos_surface_mvk(
        &self,
        instance: VkInstance,
        pCreateInfo: *const VkMacOSSurfaceCreateInfoMVK,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResultBox;

    #[cfg(feature = "VK_KHR_display")]
    unsafe fn get_physical_device_display_properties_khr(
        &self,
        physicalDevice: VkPhysicalDevice,
        pPropertyCount: *mut u32,
        pProperties: *mut VkDisplayPropertiesKHR,
    ) -> VkResultBox;
    #[cfg(feature = "VK_KHR_display")]
    unsafe fn get_physical_device_display_plane_properties_khr(
        &self,
        physicalDevice: VkPhysicalDevice,
        pPropertyCount: *mut u32,
        pProperties: *mut VkDisplayPlanePropertiesKHR,
    ) -> VkResultBox;
    #[cfg(feature = "VK_KHR_display")]
    unsafe fn get_display_plane_supported_displays_khr(
        &self,
        physicalDevice: VkPhysicalDevice,
        planeIndex: u32,
        pDisplayCount: *mut u32,
        pDisplays: *mut VkDisplayKHR,
    ) -> VkResultBox;
    #[cfg(feature = "VK_KHR_display")]
    unsafe fn get_display_mode_properties_khr(
        &self,
        physicalDevice: VkPhysicalDevice,
        display: VkDisplayKHR,
        pPropertyCount: *mut u32,
        pProperties: *mut VkDisplayModePropertiesKHR,
    ) -> VkResultBox;
    #[cfg(feature = "VK_KHR_display")]
    unsafe fn create_display_mode_khr(
        &self,
        physicalDevice: VkPhysicalDevice,
        display: VkDisplayKHR,
        pCreateInfo: *const VkDisplayModeCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pMode: *mut VkDisplayModeKHR,
    ) -> VkResultBox;
    #[cfg(feature = "VK_KHR_display")]
    unsafe fn get_display_plane_capabilities_khr(
        &self,
        physicalDevice: VkPhysicalDevice,
        mode: VkDisplayModeKHR,
        planeIndex: u32,
        pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR,
    ) -> VkResultBox;
    #[cfg(feature = "VK_KHR_display")]
    unsafe fn create_display_plane_surface_khr(
        &self,
        instance: VkInstance,
        pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR,
    ) -> VkResultBox;

    #[cfg(feature = "VK_EXT_sample_locations")]
    unsafe fn get_physical_device_multisample_properties_ext(
        &self,
        physicalDevice: VkPhysicalDevice,
        samples: VkSampleCountFlags,
        pMultisampleProperties: *mut VkMultisamplePropertiesEXT,
    );
    #[cfg(feature = "VK_EXT_sample_locations")]
    unsafe fn cmd_set_sample_locations_ext(
        &self,
        commandBuffer: VkCommandBuffer,
        pSampleLocationsInfo: *const VkSampleLocationsInfoEXT,
    );

    #[cfg(feature = "VK_EXT_image_drm_format_modifier")]
    unsafe fn get_image_drm_format_modifier_properties_ext(
        &self,
        device: VkDevice,
        image: VkImage,
        properties: *mut VkImageDrmFormatModifierPropertiesEXT,
    ) -> VkResultBox;
}

pub struct Resolver(#[cfg(feature = "DynamicLoaded")] Library);
#[cfg(not(feature = "CustomResolver"))]
impl Resolver {
    pub fn get<'a>() -> &'a Self {
        STATIC_RESOLVER_INITIALIZED.with(|f| {
            if !*f.borrow() {
                let _ = STATIC_RESOLVER.compare_exchange(
                    std::ptr::null_mut(),
                    Box::into_raw(Box::new(Self::new())),
                    Ordering::SeqCst,
                    Ordering::Relaxed,
                );
                *f.borrow_mut() = true;
            }
        });
        unsafe { &*STATIC_RESOLVER.load(Ordering::Relaxed) }
    }

    #[cfg(feature = "DynamicLoaded")]
    fn new() -> Self {
        #[cfg(target_os = "macos")]
        fn libname() -> std::path::PathBuf {
            let mut exepath = std::env::current_exe().unwrap();
            exepath.pop();
            exepath.push("libvulkan.dylib");
            return exepath;
        }
        #[cfg(windows)]
        fn libname() -> &'static str {
            "vulkan-1.dll"
        }
        #[cfg(not(any(target_os = "macos", windows)))]
        fn libname() -> &'static str {
            "libvulkan.so"
        }
        Library::new(&libname())
            .map(Resolver)
            .expect(&format!("Unable to open libvulkan: {:?}", libname()))
    }
    #[cfg(not(feature = "DynamicLoaded"))]
    fn new() -> Self {
        Resolver()
    }
}
#[cfg(feature = "CustomRenderer")]
impl Resolver {
    pub fn get<'a>() -> &'a ResolverInterface {
        unsafe { &*STATIC_RESOLVER.load(Ordering::Relaxed) }
    }
}

#[cfg(all(not(feature = "CustomResolver"), feature = "DynamicLoaded"))]
impl ResolverInterface2 for Resolver {
    unsafe fn load_symbol_unconstrainted<T: FromPtr>(&self, name: &[u8]) -> T {
        T::from_ptr(self.0.get::<T>(name).unwrap().into_raw().into_raw())
    }
}

// #[cfg(not(feature = "CustomResolver"))]
// impl ResolverInterface for Resolver {
//     WrapAPI!(
//         get_physical_device_memory_properties = vkGetPhysicalDeviceMemoryProperties(
//             physicalDevice: VkPhysicalDevice,
//             pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties
//         )
//     );
//     WrapAPI!(get_instance_proc_addr = vkGetInstanceProcAddr(instance: VkInstance, pName: *const c_char) -> Option<PFN_vkVoidFunction>);
//     WrapAPI!(get_device_proc_addr = vkGetDeviceProcAddr(device: VkDevice, pName: *const c_char) -> Option<PFN_vkVoidFunction>);
//     WrapAPI!(create_device = vkCreateDevice(physicalDevice: VkPhysicalDevice, pCreateInfo: *const VkDeviceCreateInfo, pAllocator: *const VkAllocationCallbacks, pDevice: *mut VkDevice) -> VkResult);
//     WrapAPI!(destroy_device = vkDestroyDevice(device: VkDevice, pAllocator: *const VkAllocationCallbacks));
//     WrapAPI!(enumerate_instance_extension_properties = vkEnumerateInstanceExtensionProperties(pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult);
//     WrapAPI!(enumerate_device_extension_properties = vkEnumerateDeviceExtensionProperties(physicalDevice: VkPhysicalDevice, pLayerName: *const c_char, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult);
//     WrapAPI!(enumerate_instance_layer_properties = vkEnumerateInstanceLayerProperties(pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult);
//     WrapAPI!(enumerate_device_layer_properties = vkEnumerateDeviceLayerProperties(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult);
//     WrapAPI!(
//         get_device_queue = vkGetDeviceQueue(
//             device: VkDevice,
//             queueFamilyIndex: u32,
//             queueIndex: u32,
//             pQueue: *mut VkQueue
//         )
//     );
//     WrapAPI!(queue_submit = vkQueueSubmit(queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo, fence: VkFence) -> VkResult);
//     WrapAPI!(queue_wait_idle = vkQueueWaitIdle(queue: VkQueue) -> VkResult);
//     WrapAPI!(device_wait_idle = vkDeviceWaitIdle(device: VkDevice) -> VkResult);
//     WrapAPI!(allocate_memory = vkAllocateMemory(device: VkDevice, pAllocateInfo: *const VkMemoryAllocateInfo, pAllocator: *const VkAllocationCallbacks, pMemory: *mut VkDeviceMemory) -> VkResult);
//     WrapAPI!(
//         free_memory = vkFreeMemory(
//             device: VkDevice,
//             memory: VkDeviceMemory,
//             pAllocator: *const VkAllocationCallbacks
//         )
//     );
//     WrapAPI!(map_memory = vkMapMemory(device: VkDevice, memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize, flags: VkMemoryMapFlags, ppData: *mut *mut c_void) -> VkResult);
//     WrapAPI!(unmap_memory = vkUnmapMemory(device: VkDevice, memory: VkDeviceMemory));
//     WrapAPI!(flush_mapped_memory_ranges = vkFlushMappedMemoryRanges(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult);
//     WrapAPI!(invalidate_mapped_memory_ranges = vkInvalidateMappedMemoryRanges(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult);
//     WrapAPI!(
//         get_device_memory_commitment = vkGetDeviceMemoryCommitment(
//             device: VkDevice,
//             memory: VkDeviceMemory,
//             pCommittedMemoryInBytes: *mut VkDeviceSize
//         )
//     );
//     WrapAPI!(bind_buffer_memory = vkBindBufferMemory(device: VkDevice, buffer: VkBuffer, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult);
//     WrapAPI!(bind_image_memory = vkBindImageMemory(device: VkDevice, image: VkImage, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult);
//     WrapAPI!(
//         get_buffer_memory_requirements = vkGetBufferMemoryRequirements(
//             device: VkDevice,
//             buffer: VkBuffer,
//             pMemoryRequirements: *mut VkMemoryRequirements
//         )
//     );
//     WrapAPI!(
//         get_image_memory_requirements = vkGetImageMemoryRequirements(
//             device: VkDevice,
//             image: VkImage,
//             pMemoryRequirements: *mut VkMemoryRequirements
//         )
//     );
//     WrapAPI!(
//         get_image_sparse_memory_requirements = vkGetImageSparseMemoryRequirements(
//             device: VkDevice,
//             image: VkImage,
//             pSparseMemoryRequirementCount: *mut u32,
//             pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements
//         )
//     );
//     WrapAPI!(
//         get_physical_device_sparse_image_format_properties = vkGetPhysicalDeviceSparseImageFormatProperties(
//             physicalDevice: VkPhysicalDevice,
//             format: VkFormat,
//             _type: VkImageType,
//             samples: VkSampleCountFlags,
//             usage: VkImageUsageFlags,
//             tiling: VkImageTiling,
//             pPropertyCount: *mut u32,
//             pProperties: *mut VkSparseImageFormatProperties
//         )
//     );
//     WrapAPI!(queue_bind_sparse = vkQueueBindSparse(queue: VkQueue, bindInfoCount: u32, pBindInfo: *const VkBindSparseInfo, fence: VkFence) -> VkResult);
//     WrapAPI!(create_fence = vkCreateFence(device: VkDevice, pCreateInfo: *const VkFenceCreateInfo, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence) -> VkResult);
//     WrapAPI!(
//         destroy_fence = vkDestroyFence(
//             device: VkDevice,
//             fence: VkFence,
//             pAllocator: *const VkAllocationCallbacks
//         )
//     );
//     WrapAPI!(reset_fences = vkResetFences(device: VkDevice, fenceCount: u32, pFences: *const VkFence) -> VkResult);
//     WrapAPI!(get_fence_status = vkGetFenceStatus(device: VkDevice, fence: VkFence) -> VkResult);
//     WrapAPI!(wait_for_fences = vkWaitForFences(device: VkDevice, fenceCount: u32, pFences: *const VkFence, waitAll: VkBool32, timeout: u64) -> VkResult);
//     WrapAPI!(create_semaphore = vkCreateSemaphore(device: VkDevice, pCreateInfo: *const VkSemaphoreCreateInfo, pAllocator: *const VkAllocationCallbacks, pSemaphore: *mut VkSemaphore) -> VkResult);
//     WrapAPI!(
//         destroy_semaphore = vkDestroySemaphore(
//             device: VkDevice,
//             semaphore: VkSemaphore,
//             pAllocator: *const VkAllocationCallbacks
//         )
//     );
//     WrapAPI!(create_event = vkCreateEvent(device: VkDevice, pCreateInfo: *const VkEventCreateInfo, pAllocator: *const VkAllocationCallbacks, pEvent: *mut VkEvent) -> VkResult);
//     WrapAPI!(
//         destroy_event = vkDestroyEvent(
//             device: VkDevice,
//             event: VkEvent,
//             pAllocator: *const VkAllocationCallbacks
//         )
//     );
//     WrapAPI!(get_event_status = vkGetEventStatus(device: VkDevice, event: VkEvent) -> VkResult);
//     WrapAPI!(set_event = vkSetEvent(device: VkDevice, event: VkEvent) -> VkResult);
//     WrapAPI!(reset_event = vkResetEvent(device: VkDevice, event: VkEvent) -> VkResult);
//     WrapAPI!(create_query_pool = vkCreateQueryPool(device: VkDevice, pCreateInfo: *const VkQueryPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pQueryPool: *mut VkQueryPool) -> VkResult);
//     WrapAPI!(
//         destroy_query_pool = vkDestroyQueryPool(
//             device: VkDevice,
//             queryPool: VkQueryPool,
//             pAllocator: *const VkAllocationCallbacks
//         )
//     );
//     WrapAPI!(get_query_pool_results = vkGetQueryPoolResults(device: VkDevice, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dataSize: size_t, pData: *mut c_void, stride: VkDeviceSize, flags: VkQueryResultFlags) -> VkResult);
//     WrapAPI!(create_buffer = vkCreateBuffer(device: VkDevice, pCreateInfo: *const VkBufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pBuffer: *mut VkBuffer) -> VkResult);
//     WrapAPI!(
//         destroy_buffer = vkDestroyBuffer(
//             device: VkDevice,
//             buffer: VkBuffer,
//             pAllocator: *const VkAllocationCallbacks
//         )
//     );
//     WrapAPI!(create_buffer_view = vkCreateBufferView(device: VkDevice, pCreateInfo: *const VkBufferViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkBufferView) -> VkResult);
//     WrapAPI!(
//         destroy_buffer_view = vkDestroyBufferView(
//             device: VkDevice,
//             bufferView: VkBufferView,
//             pAllocator: *const VkAllocationCallbacks
//         )
//     );
//     WrapAPI!(create_image = vkCreateImage(device: VkDevice, pCreateInfo: *const VkImageCreateInfo, pAllocator: *const VkAllocationCallbacks, pImage: *mut VkImage) -> VkResult);
//     WrapAPI!(
//         destroy_image = vkDestroyImage(
//             device: VkDevice,
//             image: VkImage,
//             pAllocator: *const VkAllocationCallbacks
//         )
//     );
//     WrapAPI!(
//         get_image_subresource_layout = vkGetImageSubresourceLayout(
//             device: VkDevice,
//             image: VkImage,
//             pSubresource: *const VkImageSubresource,
//             pLayout: *mut VkSubresourceLayout
//         )
//     );
//     WrapAPI!(create_image_view = vkCreateImageView(device: VkDevice, pCreateInfo: *const VkImageViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkImageView) -> VkResult);
//     WrapAPI!(
//         destroy_image_view = vkDestroyImageView(
//             device: VkDevice,
//             imageView: VkImageView,
//             pAllocator: *const VkAllocationCallbacks
//         )
//     );
//     WrapAPI!(create_shader_module = vkCreateShaderModule(device: VkDevice, pCreateInfo: *const VkShaderModuleCreateInfo, pAllocator: *const VkAllocationCallbacks, pShaderModule: *mut VkShaderModule) -> VkResult);
//     WrapAPI!(
//         destroy_shader_module = vkDestroyShaderModule(
//             device: VkDevice,
//             shaderModule: VkShaderModule,
//             pAllocator: *const VkAllocationCallbacks
//         )
//     );
//     WrapAPI!(create_pipeline_cache = vkCreatePipelineCache(device: VkDevice, pCreateInfo: *const VkPipelineCacheCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineCache: *mut VkPipelineCache) -> VkResult);
//     WrapAPI!(
//         destroy_pipeline_cache = vkDestroyPipelineCache(
//             device: VkDevice,
//             pipelineCache: VkPipelineCache,
//             pAllocator: *const VkAllocationCallbacks
//         )
//     );
//     WrapAPI!(get_pipeline_cache_data = vkGetPipelineCacheData(device: VkDevice, pipelineCache: VkPipelineCache, pDataSize: *mut size_t, pData: *mut c_void) -> VkResult);
//     WrapAPI!(merge_pipeline_caches = vkMergePipelineCaches(device: VkDevice, dstCache: VkPipelineCache, srcCacheCount: u32, pSrcCaches: *const VkPipelineCache) -> VkResult);
//     WrapAPI!(create_graphics_pipelines = vkCreateGraphicsPipelines(device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: u32, pCreateInfos: *const VkGraphicsPipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult);
//     WrapAPI!(create_compute_pipelines = vkCreateComputePipelines(device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: u32, pCreateInfos: *const VkComputePipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult);
//     WrapAPI!(
//         destroy_pipeline = vkDestroyPipeline(
//             device: VkDevice,
//             pipeline: VkPipeline,
//             pAllocator: *const VkAllocationCallbacks
//         )
//     );
//     WrapAPI!(create_pipeline_layout = vkCreatePipelineLayout(device: VkDevice, pCreateInfo: *const VkPipelineLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineLayout: *mut VkPipelineLayout) -> VkResult);
//     WrapAPI!(
//         destroy_pipeline_layout = vkDestroyPipelineLayout(
//             device: VkDevice,
//             pipelineLayout: VkPipelineLayout,
//             pAllocator: *const VkAllocationCallbacks
//         )
//     );
//     WrapAPI!(create_sampler = vkCreateSampler(device: VkDevice, pCreateInfo: *const VkSamplerCreateInfo, pAllocator: *const VkAllocationCallbacks, pSampler: *mut VkSampler) -> VkResult);
//     WrapAPI!(
//         destroy_sampler = vkDestroySampler(
//             device: VkDevice,
//             sampler: VkSampler,
//             pAllocator: *const VkAllocationCallbacks
//         )
//     );
//     WrapAPI!(create_descriptor_set_layout = vkCreateDescriptorSetLayout(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pSetLayout: *mut VkDescriptorSetLayout) -> VkResult);
//     WrapAPI!(
//         destroy_descriptor_set_layout = vkDestroyDescriptorSetLayout(
//             device: VkDevice,
//             descriptorSetLayout: VkDescriptorSetLayout,
//             pAllocator: *const VkAllocationCallbacks
//         )
//     );
//     WrapAPI!(create_descriptor_pool = vkCreateDescriptorPool(device: VkDevice, pCreateInfo: *const VkDescriptorPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pDescriptorPool: *mut VkDescriptorPool) -> VkResult);
//     WrapAPI!(
//         destroy_descriptor_pool = vkDestroyDescriptorPool(
//             device: VkDevice,
//             descriptorPool: VkDescriptorPool,
//             pAllocator: *const VkAllocationCallbacks
//         )
//     );
//     WrapAPI!(reset_descriptor_pool = vkResetDescriptorPool(device: VkDevice, descriptorPool: VkDescriptorPool, flags: VkDescriptorPoolResetFlags) -> VkResult);
//     WrapAPI!(allocate_descriptor_sets = vkAllocateDescriptorSets(device: VkDevice, pAllocateInfo: *const VkDescriptorSetAllocateInfo, pDescriptorSets: *mut VkDescriptorSet) -> VkResult);
//     WrapAPI!(free_descriptor_sets = vkFreeDescriptorSets(device: VkDevice, descriptorPool: VkDescriptorPool, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet) -> VkResult);
//     WrapAPI!(
//         update_descriptor_sets = vkUpdateDescriptorSets(
//             device: VkDevice,
//             descriptorWriteCount: u32,
//             pDescriptorWrites: *const VkWriteDescriptorSet,
//             descriptorCopyCount: u32,
//             pDescriptorCopies: *const VkCopyDescriptorSet
//         )
//     );
//     WrapAPI!(create_framebuffer = vkCreateFramebuffer(device: VkDevice, pCreateInfo: *const VkFramebufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pFramebuffer: *mut VkFramebuffer) -> VkResult);
//     WrapAPI!(
//         destroy_framebuffer = vkDestroyFramebuffer(
//             device: VkDevice,
//             framebuffer: VkFramebuffer,
//             pAllocator: *const VkAllocationCallbacks
//         )
//     );
//     WrapAPI!(create_render_pass = vkCreateRenderPass(device: VkDevice, pCreateInfo: *const VkRenderPassCreateInfo, pAllocator: *const VkAllocationCallbacks, pRenderPass: *mut VkRenderPass) -> VkResult);
//     WrapAPI!(
//         destroy_render_pass = vkDestroyRenderPass(
//             device: VkDevice,
//             renderPass: VkRenderPass,
//             pAllocator: *const VkAllocationCallbacks
//         )
//     );
//     WrapAPI!(
//         get_render_area_granularity = vkGetRenderAreaGranularity(
//             device: VkDevice,
//             renderPass: VkRenderPass,
//             pGranularity: *mut VkExtent2D
//         )
//     );
//     WrapAPI!(create_command_pool = vkCreateCommandPool(device: VkDevice, pCreateInfo: *const VkCommandPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pCommandPool: *mut VkCommandPool) -> VkResult);
//     WrapAPI!(
//         destroy_command_pool = vkDestroyCommandPool(
//             device: VkDevice,
//             commandPool: VkCommandPool,
//             pAllocator: *const VkAllocationCallbacks
//         )
//     );
//     WrapAPI!(reset_command_pool = vkResetCommandPool(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolResetFlags) -> VkResult);
//     WrapAPI!(allocate_command_buffers = vkAllocateCommandBuffers(device: VkDevice, pAllocateInfo: *const VkCommandBufferAllocateInfo, pCommandBuffers: *mut VkCommandBuffer) -> VkResult);
//     WrapAPI!(
//         free_command_buffers = vkFreeCommandBuffers(
//             device: VkDevice,
//             commandPool: VkCommandPool,
//             commandBufferCount: u32,
//             pCommandBuffers: *const VkCommandBuffer
//         )
//     );
//     WrapAPI!(begin_command_buffer = vkBeginCommandBuffer(commandBuffer: VkCommandBuffer, pBeginInfo: *const VkCommandBufferBeginInfo) -> VkResult);
//     WrapAPI!(end_command_buffer = vkEndCommandBuffer(commandBuffer: VkCommandBuffer) -> VkResult);
//     WrapAPI!(reset_command_buffer = vkResetCommandBuffer(commandBuffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResult);

//     WrapAPI!(
//         cmd_bind_pipeline = vkCmdBindPipeline(
//             commandBuffer: VkCommandBuffer,
//             pipelineBindPoint: VkPipelineBindPoint,
//             pipeline: VkPipeline
//         )
//     );
//     WrapAPI!(
//         cmd_set_viewport = vkCmdSetViewport(
//             commandBuffer: VkCommandBuffer,
//             firstViewport: u32,
//             viewportCount: u32,
//             pViewports: *const VkViewport
//         )
//     );
//     WrapAPI!(
//         cmd_set_scissor = vkCmdSetScissor(
//             commandBuffer: VkCommandBuffer,
//             firstScissor: u32,
//             scissorCount: u32,
//             pScissors: *const VkRect2D
//         )
//     );
//     WrapAPI!(cmd_set_line_width = vkCmdSetLineWidth(commandBuffer: VkCommandBuffer, lineWidth: c_float));
//     WrapAPI!(
//         cmd_set_depth_bias = vkCmdSetDepthBias(
//             commandBuffer: VkCommandBuffer,
//             depthBiasConstantFactor: c_float,
//             depthBiasClamp: c_float,
//             depthBiasSlopeFactor: c_float
//         )
//     );
//     WrapAPI!(
//         cmd_set_blend_constants =
//             vkCmdSetBlendConstants(commandBuffer: VkCommandBuffer, blendConstants: *const c_float)
//     );
//     WrapAPI!(
//         cmd_set_depth_bounds = vkCmdSetDepthBounds(
//             commandBuffer: VkCommandBuffer,
//             minDepthBounds: c_float,
//             maxDepthBounds: c_float
//         )
//     );
//     WrapAPI!(
//         cmd_set_stencil_compare_mask = vkCmdSetStencilCompareMask(
//             commandBuffer: VkCommandBuffer,
//             faceMask: VkStencilFaceFlags,
//             compareMask: u32
//         )
//     );
//     WrapAPI!(
//         cmd_set_stencil_write_mask = vkCmdSetStencilWriteMask(
//             commandBuffer: VkCommandBuffer,
//             faceMask: VkStencilFaceFlags,
//             writeMask: u32
//         )
//     );
//     WrapAPI!(
//         cmd_set_stencil_reference = vkCmdSetStencilReference(
//             commandBuffer: VkCommandBuffer,
//             faceMask: VkStencilFaceFlags,
//             reference: u32
//         )
//     );
//     WrapAPI!(
//         cmd_bind_descriptor_sets = vkCmdBindDescriptorSets(
//             commandBuffer: VkCommandBuffer,
//             pipelineBindPoint: VkPipelineBindPoint,
//             layout: VkPipelineLayout,
//             firstSet: u32,
//             descriptorSetCount: u32,
//             pDescriptorSets: *const VkDescriptorSet,
//             dynamicOffsetCount: u32,
//             pDynamicOffsets: *const u32
//         )
//     );
//     WrapAPI!(
//         cmd_bind_index_buffer = vkCmdBindIndexBuffer(
//             commandBuffer: VkCommandBuffer,
//             buffer: VkBuffer,
//             offset: VkDeviceSize,
//             indexType: VkIndexType
//         )
//     );
//     WrapAPI!(
//         cmd_bind_vertex_buffers = vkCmdBindVertexBuffers(
//             commandBuffer: VkCommandBuffer,
//             firstBinding: u32,
//             bindingCount: u32,
//             pBuffers: *const VkBuffer,
//             pOffsets: *const VkDeviceSize
//         )
//     );
//     WrapAPI!(
//         cmd_draw = vkCmdDraw(
//             commandBuffer: VkCommandBuffer,
//             vertexCount: u32,
//             instanceCount: u32,
//             firstVertex: u32,
//             firstInstance: u32
//         )
//     );
//     WrapAPI!(
//         cmd_draw_indexed = vkCmdDrawIndexed(
//             commandBuffer: VkCommandBuffer,
//             indexCount: u32,
//             instanceCount: u32,
//             firstIndex: u32,
//             vertexOffset: i32,
//             firstInstance: u32
//         )
//     );
//     WrapAPI!(
//         cmd_draw_indirect = vkCmdDrawIndirect(
//             commandBuffer: VkCommandBuffer,
//             buffer: VkBuffer,
//             offset: VkDeviceSize,
//             drawCount: u32,
//             stride: u32
//         )
//     );
//     WrapAPI!(
//         cmd_draw_indexed_indirect = vkCmdDrawIndexedIndirect(
//             commandBuffer: VkCommandBuffer,
//             buffer: VkBuffer,
//             offset: VkDeviceSize,
//             drawCount: u32,
//             stride: u32
//         )
//     );
//     WrapAPI!(
//         cmd_dispatch = vkCmdDispatch(
//             commandBuffer: VkCommandBuffer,
//             groupCountX: u32,
//             groupCountY: u32,
//             groupCountZ: u32
//         )
//     );
//     WrapAPI!(
//         cmd_dispatch_indirect =
//             vkCmdDispatchIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize)
//     );
//     WrapAPI!(
//         cmd_copy_buffer = vkCmdCopyBuffer(
//             commandBuffer: VkCommandBuffer,
//             srcBuffer: VkBuffer,
//             dstBuffer: VkBuffer,
//             regionCount: u32,
//             pRegions: *const VkBufferCopy
//         )
//     );
//     WrapAPI!(
//         cmd_copy_image = vkCmdCopyImage(
//             commandBuffer: VkCommandBuffer,
//             srcImage: VkImage,
//             srcImageLayout: VkImageLayout,
//             dstImage: VkImage,
//             dstImageLayout: VkImageLayout,
//             regionCount: u32,
//             pRegions: *const VkImageCopy
//         )
//     );
//     WrapAPI!(
//         cmd_blit_image = vkCmdBlitImage(
//             commandBuffer: VkCommandBuffer,
//             srcImage: VkImage,
//             srcImageLayout: VkImageLayout,
//             dstImage: VkImage,
//             dstImageLayout: VkImageLayout,
//             regionCount: u32,
//             pRegions: *const VkImageBlit,
//             filter: VkFilter
//         )
//     );
//     WrapAPI!(
//         cmd_copy_buffer_to_image = vkCmdCopyBufferToImage(
//             commandBuffer: VkCommandBuffer,
//             srcBuffer: VkBuffer,
//             dstImage: VkImage,
//             dstImageLayout: VkImageLayout,
//             regionCount: u32,
//             pRegions: *const VkBufferImageCopy
//         )
//     );
//     WrapAPI!(
//         cmd_copy_image_to_buffer = vkCmdCopyImageToBuffer(
//             commandBuffer: VkCommandBuffer,
//             srcImage: VkImage,
//             srcImageLayout: VkImageLayout,
//             dstBuffer: VkBuffer,
//             regionCount: u32,
//             pRegions: *const VkBufferImageCopy
//         )
//     );
//     WrapAPI!(
//         cmd_update_buffer = vkCmdUpdateBuffer(
//             commandBuffer: VkCommandBuffer,
//             dstBuffer: VkBuffer,
//             dstOffset: VkDeviceSize,
//             dataSize: VkDeviceSize,
//             pData: *const c_void
//         )
//     );
//     WrapAPI!(
//         cmd_fill_buffer = vkCmdFillBuffer(
//             commandBuffer: VkCommandBuffer,
//             dstBuffer: VkBuffer,
//             dstOffset: VkDeviceSize,
//             size: VkDeviceSize,
//             data: u32
//         )
//     );
//     WrapAPI!(
//         cmd_clear_color_image = vkCmdClearColorImage(
//             commandBuffer: VkCommandBuffer,
//             image: VkImage,
//             imageLayout: VkImageLayout,
//             pColor: *const VkClearColorValue,
//             rangeCount: u32,
//             pRanges: *const VkImageSubresourceRange
//         )
//     );
//     WrapAPI!(
//         cmd_clear_depth_stencil_image = vkCmdClearDepthStencilImage(
//             commandBuffer: VkCommandBuffer,
//             image: VkImage,
//             imageLayout: VkImageLayout,
//             pDepthStencil: *const VkClearDepthStencilValue,
//             rangeCount: u32,
//             pRanges: *const VkImageSubresourceRange
//         )
//     );
//     WrapAPI!(
//         cmd_clear_attachments = vkCmdClearAttachments(
//             commandBuffer: VkCommandBuffer,
//             attachmentCount: u32,
//             pAttachments: *const VkClearAttachment,
//             rectCount: u32,
//             pRects: *const VkClearRect
//         )
//     );
//     WrapAPI!(
//         cmd_resolve_image = vkCmdResolveImage(
//             commandBuffer: VkCommandBuffer,
//             srcImage: VkImage,
//             srcImageLayout: VkImageLayout,
//             dstImage: VkImage,
//             dstImageLayout: VkImageLayout,
//             regionCount: u32,
//             pRegions: *const VkImageResolve
//         )
//     );
//     WrapAPI!(
//         cmd_set_event = vkCmdSetEvent(
//             commandBuffer: VkCommandBuffer,
//             event: VkEvent,
//             stageMask: VkPipelineStageFlags
//         )
//     );
//     WrapAPI!(
//         cmd_reset_event = vkCmdResetEvent(
//             commandBuffer: VkCommandBuffer,
//             event: VkEvent,
//             stageMask: VkPipelineStageFlags
//         )
//     );
//     WrapAPI!(
//         cmd_wait_events = vkCmdWaitEvents(
//             commandBuffer: VkCommandBuffer,
//             eventCount: u32,
//             pEvents: *const VkEvent,
//             srcStageMask: VkPipelineStageFlags,
//             dstStageMask: VkPipelineStageFlags,
//             memoryBarrierCount: u32,
//             pMemoryBarriers: *const VkMemoryBarrier,
//             bufferMemoryBarrierCount: u32,
//             pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
//             imageMemoryBarrierCount: u32,
//             pImageMemoryBariers: *const VkImageMemoryBarrier
//         )
//     );
//     WrapAPI!(
//         cmd_pipeline_barrier = vkCmdPipelineBarrier(
//             commandBuffer: VkCommandBuffer,
//             srcStageMask: VkPipelineStageFlags,
//             dstStageMask: VkPipelineStageFlags,
//             dependencyFlags: VkDependencyFlags,
//             memoryBarrierCount: u32,
//             pMemoryBarriers: *const VkMemoryBarrier,
//             bufferMemoryBarrierCount: u32,
//             pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
//             imageMemoryBarrierCount: u32,
//             pImageMemoryBarriers: *const VkImageMemoryBarrier
//         )
//     );
//     WrapAPI!(
//         cmd_begin_query = vkCmdBeginQuery(
//             commandBuffer: VkCommandBuffer,
//             queryPool: VkQueryPool,
//             query: u32,
//             flags: VkQueryControlFlags
//         )
//     );
//     WrapAPI!(cmd_end_query = vkCmdEndQuery(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32));
//     WrapAPI!(
//         cmd_reset_query_pool = vkCmdResetQueryPool(
//             commandBuffer: VkCommandBuffer,
//             queryPool: VkQueryPool,
//             firstQuery: u32,
//             queryCount: u32
//         )
//     );
//     WrapAPI!(
//         cmd_write_timestamp = vkCmdWriteTimestamp(
//             commandBuffer: VkCommandBuffer,
//             pipelineStage: VkPipelineStageFlags,
//             queryPool: VkQueryPool,
//             query: u32
//         )
//     );
//     WrapAPI!(
//         cmd_copy_query_pool_results = vkCmdCopyQueryPoolResults(
//             commandBuffer: VkCommandBuffer,
//             queryPool: VkQueryPool,
//             firstQuery: u32,
//             queryCount: u32,
//             dstBuffer: VkBuffer,
//             dstOffset: VkDeviceSize,
//             stride: VkDeviceSize,
//             flags: VkQueryResultFlags
//         )
//     );
//     WrapAPI!(
//         cmd_push_constants = vkCmdPushConstants(
//             commandBuffer: VkCommandBuffer,
//             layout: VkPipelineLayout,
//             stageFlags: VkShaderStageFlags,
//             offset: u32,
//             size: u32,
//             pValues: *const c_void
//         )
//     );
//     WrapAPI!(
//         cmd_begin_render_pass = vkCmdBeginRenderPass(
//             commandBuffer: VkCommandBuffer,
//             pRenderPassBegin: *const VkRenderPassBeginInfo,
//             contents: VkSubpassContents
//         )
//     );
//     WrapAPI!(cmd_next_subpass = vkCmdNextSubpass(commandBuffer: VkCommandBuffer, contents: VkSubpassContents));
//     WrapAPI!(cmd_end_render_pass = vkCmdEndRenderPass(commandBuffer: VkCommandBuffer));
//     WrapAPI!(
//         cmd_execute_commands = vkCmdExecuteCommands(
//             commandBuffer: VkCommandBuffer,
//             commandBufferCount: u32,
//             pCommandBuffers: *const VkCommandBuffer
//         )
//     );
//     #[cfg(feature = "VK_KHR_push_descriptor")]
//     WrapAPI!(
//         cmd_push_descriptor_set_khr = vkCmdPushDescriptorSetKHR(
//             commandBuffer: VkCommandBuffer,
//             pipelineBindPoint: VkPipelineBindPoint,
//             layout: VkPipelineLayout,
//             set: u32,
//             descriptorWriteCount: u32,
//             pDescriptorWrites: *const VkWriteDescriptorSet
//         )
//     );
//     #[cfg(feature = "VK_KHR_descriptor_update_template")]
//     #[cfg(feature = "VK_KHR_push_descriptor")]
//     WrapAPI!(
//         push_descriptor_set_with_template_khr = vkCmdPushDescriptorSetWithTemplateKHR(
//             commandBuffer: VkCommandBuffer,
//             descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR,
//             layout: VkPipelineLayout,
//             set: u32,
//             pData: *const c_void
//         )
//     );
//     #[cfg(feature = "VK_EXT_debug_marker")]
//     WrapAPI!(
//         cmd_debug_marker_begin_ext = vkCmdDebugMarkerBeginEXT(
//             commandBuffer: VkCommandBuffer,
//             pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT
//         )
//     );
//     #[cfg(feature = "VK_EXT_debug_marker")]
//     WrapAPI!(cmd_debug_marker_end_ext = vkCmdDebugMarkerEndEXT(commandBuffer: VkCommandBuffer));
//     #[cfg(feature = "VK_EXT_debug_marker")]
//     WrapAPI!(
//         cmd_debug_marker_insert_ext = vkCmdDebugMarkerInsertEXT(
//             commandBuffer: VkCommandBuffer,
//             pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT
//         )
//     );
//     #[cfg(feature = "VK_AMD_draw_indirect_count")]
//     WrapAPI!(
//         cmd_draw_indirect_count_amd = vkCmdDrawIndirectCountAMD(
//             commandBuffer: VkCommandBuffer,
//             buffer: VkBuffer,
//             offset: VkDeviceSize,
//             countBuffer: VkBuffer,
//             countBufferOffset: VkDeviceSize,
//             maxDrawCount: u32,
//             stride: u32
//         )
//     );
//     #[cfg(feature = "VK_AMD_draw_indirect_count")]
//     WrapAPI!(
//         cmd_draw_indexed_indirect_count_amd = vkCmdDrawIndexedIndirectCountAMD(
//             commandBuffer: VkCommandBuffer,
//             buffer: VkBuffer,
//             offset: VkDeviceSize,
//             countBuffer: VkBuffer,
//             countBufferOffset: VkDeviceSize,
//             maxDrawCount: u32,
//             stride: u32
//         )
//     );
//     #[cfg(feature = "VK_KHX_device_group")]
//     WrapAPI!(cmd_set_device_mask_khx = vkCmdSetDeviceMaskKHX(commandBuffer: VkCommandBuffer, deviceMask: u32));
//     #[cfg(feature = "VK_KHX_device_group")]
//     WrapAPI!(
//         cmd_dispatch_base_khx = vkCmdDispatchBaseKHX(
//             commandBuffer: VkCommandBuffer,
//             baseGroupX: u32,
//             baseGroupY: u32,
//             baseGroupZ: u32,
//             groupCountX: u32,
//             groupCountY: u32,
//             groupCountZ: u32
//         )
//     );
//     #[cfg(feature = "VK_NVX_device_generated_commands")]
//     WrapAPI!(
//         cmd_process_commands_nvx = vkCmdProcessCommandsNVX(
//             commandBuffer: VkCommandBuffer,
//             pProcessCommandsInfo: *const VkCmdProcessCommandsInfoNVX
//         )
//     );
//     #[cfg(feature = "VK_NVX_device_generated_commands")]
//     WrapAPI!(
//         cmd_reserve_space_for_commands_nvx = vkCmdReserveSpaceForCommandsNVX(
//             commandBuffer: VkCommandBuffer,
//             pReserveSpaceInfo: *const VkCmdReserveSpaceForCommandsInfoNVX
//         )
//     );
//     #[cfg(feature = "VK_NV_clip_space_w_scaling")]
//     WrapAPI!(
//         cmd_set_viewport_w_scaling_nv = vkCmdSetViewportWScalingNV(
//             commandBuffer: VkCommandBuffer,
//             firstViewport: u32,
//             viewportCount: u32,
//             pViewportWScalings: *const VkViewportWScalingNV
//         )
//     );
//     #[cfg(feature = "VK_EXT_discard_rectangles")]
//     WrapAPI!(
//         cmd_discard_rectangle_ext = vkCmdDiscardRectangleEXT(
//             commandBuffer: VkCommandBuffer,
//             firstDiscardRectangle: u32,
//             discardRectangleCount: u32,
//             pDiscardRectangles: *const VkRect2D
//         )
//     );

//     #[cfg(feature = "VK_KHR_surface")]
//     WrapAPI!(
//         destroy_surface_khr = vkDestroySurfaceKHR(
//             instance: VkInstance,
//             surface: VkSurfaceKHR,
//             pAllocator: *const VkAllocationCallbacks
//         )
//     );
//     #[cfg(feature = "VK_KHR_surface")]
//     WrapAPI!(get_physical_device_surface_support_khr = vkGetPhysicalDeviceSurfaceSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, surface: VkSurfaceKHR, pSupported: *mut VkBool32) -> VkResult);
//     #[cfg(feature = "VK_KHR_surface")]
//     WrapAPI!(get_physical_device_surface_capabilities_khr = vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR) -> VkResult);
//     #[cfg(feature = "VK_KHR_surface")]
//     WrapAPI!(get_physical_device_surface_formats_khr = vkGetPhysicalDeviceSurfaceFormatsKHR(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceFormatCount: *mut u32, pSurfaceFormats: *mut VkSurfaceFormatKHR) -> VkResult);
//     #[cfg(feature = "VK_KHR_surface")]
//     WrapAPI!(get_physical_device_surface_present_modes_khr = vkGetPhysicalDeviceSurfacePresentModesKHR(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pPresentModeCount: *mut u32, pPresentModes: *mut VkPresentModeKHR) -> VkResult);
//     #[cfg(feature = "VK_KHR_surface")]
//     #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
//     WrapAPI!(get_physical_device_surface_capabilities2_khr = vkGetPhysicalDeviceSurfaceCapabilities2KHR(physicalDevice: VkPhysicalDevice, surface_info: *const VkPhysicalDeviceSurfaceInfo2KHR, surface_capabilities: *mut VkSurfaceCapabilities2KHR) -> VkResult);

//     #[cfg(feature = "VK_KHR_swapchain")]
//     WrapAPI!(create_swapchain_khr = vkCreateSwapchainKHR(device: VkDevice, pCreateInfo: *const VkSwapchainCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSwapchain: *mut VkSwapchainKHR) -> VkResult);
//     #[cfg(feature = "VK_KHR_swapchain")]
//     WrapAPI!(
//         destroy_swapchain_khr = vkDestroySwapchainKHR(
//             device: VkDevice,
//             swapchain: VkSwapchainKHR,
//             pAllocator: *const VkAllocationCallbacks
//         )
//     );
//     #[cfg(feature = "VK_KHR_swapchain")]
//     WrapAPI!(get_swapchain_images_khr = vkGetSwapchainImagesKHR(device: VkDevice, swapchain: VkSwapchainKHR, pSwapchainImageCount: *mut u32, pSwapchainImages: *mut VkImage) -> VkResult);
//     #[cfg(feature = "VK_KHR_swapchain")]
//     WrapAPI!(acquire_next_image_khr = vkAcquireNextImageKHR(device: VkDevice, swapchain: VkSwapchainKHR, timeout: u64, semaphore: VkSemaphore, fence: VkFence, pImageIndex: *mut u32) -> VkResult);
//     #[cfg(feature = "VK_KHR_swapchain")]
//     WrapAPI!(queue_present_khr = vkQueuePresentKHR(queue: VkQueue, pPresentInfo: *const VkPresentInfoKHR) -> VkResult);

//     #[cfg(feature = "VK_KHR_xlib_surface")]
//     WrapAPI!(create_xlib_surface_khr = vkCreateXlibSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkXlibSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult);
//     #[cfg(feature = "VK_KHR_xlib_surface")]
//     WrapAPI!(get_physical_device_xlib_presentation_support_khr = vkGetPhysicalDeviceXlibPresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, dpy: *mut Display, visualID: VisualID) -> VkBool32);

//     #[cfg(feature = "VK_KHR_xcb_surface")]
//     WrapAPI!(create_xcb_surface_khr = vkCreateXcbSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkXcbSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult);
//     #[cfg(feature = "VK_KHR_xcb_surface")]
//     WrapAPI!(get_physical_device_xcb_presentation_support_khr = vkGetPhysicalDeviceXcbPresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, connection: *mut xcb_connection_t, visual_id: xcb::x::Visualid) -> VkBool32);

//     #[cfg(feature = "VK_KHR_wayland_surface")]
//     WrapAPI!(create_wayland_surface_khr = vkCreateWaylandSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkWaylandSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult);
//     #[cfg(feature = "VK_KHR_wayland_surface")]
//     WrapAPI!(get_physical_device_wayland_presentation_support_khr = vkGetPhysicalDeviceWaylandPresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, display: *mut wayland_client::sys::wl_display) -> VkBool32);

//     #[cfg(feature = "VK_KHR_android_surface")]
//     WrapAPI!(create_android_surface_khr = vkCreateAndroidSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkAndroidSurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult);

//     #[cfg(feature = "VK_KHR_win32_surface")]
//     WrapAPI!(create_win32_surface_khr = vkCreateWin32SurfaceKHR(instance: VkInstance, pCreateInfo: *const VkWin32SurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult);
//     #[cfg(feature = "VK_KHR_win32_surface")]
//     WrapAPI!(get_physical_device_win32_presentation_support_khr = vkGetPhysicalDeviceWin32PresentationSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32) -> VkBool32);

//     #[cfg(feature = "VK_MVK_macos_surface")]
//     WrapAPI!(create_macos_surface_mvk = vkCreateMacOSSurfaceMVK(instance: VkInstance, pCreateInfo: *const VkMacOSSurfaceCreateInfoMVK, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult);

//     #[cfg(feature = "VK_KHR_display")]
//     WrapAPI!(get_physical_device_display_properties_khr = vkGetPhysicalDeviceDisplayPropertiesKHR(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPropertiesKHR) -> VkResult);
//     #[cfg(feature = "VK_KHR_display")]
//     WrapAPI!(get_physical_device_display_plane_properties_khr = vkGetPhysicalDeviceDisplayPlanePropertiesKHR(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPlanePropertiesKHR) -> VkResult);
//     #[cfg(feature = "VK_KHR_display")]
//     WrapAPI!(get_display_plane_supported_displays_khr = vkGetDisplayPlaneSupportedDisplaysKHR(physicalDevice: VkPhysicalDevice, planeIndex: u32, pDisplayCount: *mut u32, pDisplays: *mut VkDisplayKHR) -> VkResult);
//     #[cfg(feature = "VK_KHR_display")]
//     WrapAPI!(get_display_mode_properties_khr = vkGetDisplayModePropertiesKHR(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR, pPropertyCount: *mut u32, pProperties: *mut VkDisplayModePropertiesKHR) -> VkResult);
//     #[cfg(feature = "VK_KHR_display")]
//     WrapAPI!(create_display_mode_khr = vkCreateDisplayModeKHR(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR, pCreateInfo: *const VkDisplayModeCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pMode: *mut VkDisplayModeKHR) -> VkResult);
//     #[cfg(feature = "VK_KHR_display")]
//     WrapAPI!(get_display_plane_capabilities_khr = vkGetDisplayPlaneCapabilitiesKHR(physicalDevice: VkPhysicalDevice, mode: VkDisplayModeKHR, planeIndex: u32, pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR) -> VkResult);
//     #[cfg(feature = "VK_KHR_display")]
//     WrapAPI!(create_display_plane_surface_khr = vkCreateDisplayPlaneSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult);

//     #[cfg(feature = "VK_EXT_sample_locations")]
//     WrapAPI!(
//         get_physical_device_multisample_properties_ext = vkGetPhysicalDeviceMultisamplePropertiesEXT(
//             physicalDevice: VkPhysicalDevice,
//             samples: VkSampleCountFlags,
//             pMultisampleProperties: *mut VkMultisamplePropertiesEXT
//         )
//     );
//     #[cfg(feature = "VK_EXT_sample_locations")]
//     WrapAPI!(
//         cmd_set_sample_locations_ext = vkCmdSetSampleLocationsEXT(
//             commandBuffer: VkCommandBuffer,
//             pSampleLocationsInfo: *const VkSampleLocationsInfoEXT
//         )
//     );

//     #[cfg(feature = "VK_EXT_image_drm_format_modifier")]
//     WrapAPI!(get_image_drm_format_modifier_properties_ext = vkGetImageDrmFormatModifierPropertiesEXT(
//         device: VkDevice,
//         image: VkImage,
//         properties: *mut VkImageDrmFormatModifierPropertiesEXT
//     ) -> VkResult);
// }
