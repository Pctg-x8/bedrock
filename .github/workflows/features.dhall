let PlatformIndependent =
      [ "VK_EXT_blend_operation_advanced"
      , "VK_AMD_buffer_marker"
      , "VK_NV_clip_space_w_scaling"
      , "VK_EXT_conservative_rasterization"
      , "VK_EXT_debug_marker"
      , "VK_EXT_debug_report"
      , "VK_EXT_debug_utils"
      , "VK_NV_dedicated_allocation"
      , "VK_EXT_depth_range_unrestricted"
      , "VK_KHR_descriptor_update_template"
      , "VK_NVX_device_generated_commands"
      , "VK_KHX_device_group_creation"
      , "VK_KHX_device_group"
      , "VK_EXT_direct_mode_display"
      , "VK_EXT_discard_rectangles"
      , "VK_EXT_display_control"
      , "VK_KHR_display"
      , "VK_EXT_display_surface_counter"
      , "VK_KHR_display_swapchain"
      , "VK_GOOGLE_display_timing"
      , "VK_AMD_draw_indirect_count"
      , "VK_NV_external_memory_capabilities"
      , "VK_EXT_external_memory_host"
      , "VK_NV_external_memory"
      , "VK_NV_fill_rectangle"
      , "VK_IMG_filter_cubic"
      , "VK_IMG_format_pvrtc"
      , "VK_NV_fragment_coverage_to_color"
      , "VK_NV_framebuffer_mixed_samples"
      , "VK_EXT_full_screen_exclusive"
      , "VK_AMD_gcn_shader"
      , "VK_NV_geometry_shader_passthrough"
      , "VK_KHR_get_physical_device_properties2"
      , "VK_KHR_get_surface_capabilities2"
      , "VK_EXT_global_priority"
      , "VK_NV_glsl_shader"
      , "VK_AMD_gpu_shader_half_float"
      , "VK_AMD_gpu_shader_int16"
      , "VK_EXT_hdr_metadata"
      , "VK_KHR_image_format_list"
      , "VK_KHR_incremental_present"
      , "VK_KHR_maintenance1"
      , "VK_AMD_mixed_attachment_samples"
      , "VK_NVX_multiview_per_view_attributes"
      , "VK_AMD_negative_viewport_height"
      , "VK_EXT_post_depth_coverage"
      , "VK_KHR_push_descriptor"
      , "VK_EXT_queue_family_foreign"
      , "VK_AMD_rasterization_order"
      , "VK_KHR_relaxed_block_layout"
      , "VK_EXT_sample_locations"
      , "VK_NV_sample_mask_override_coverage"
      , "VK_EXT_sampler_filter_minmax"
      , "VK_KHR_sampler_mirror_clamp_to_edge"
      , "VK_AMD_shader_ballot"
      , "VK_KHR_shader_draw_parameters"
      , "VK_AMD_shader_explicit_vertex_parameter"
      , "VK_AMD_shader_fragment_mask"
      , "VK_EXT_shader_group_ballot"
      , "VK_AMD_shader_image_load_store_lod"
      , "VK_AMD_shader_info"
      , "VK_EXT_shader_subgroup_vote"
      , "VK_AMD_shader_trinary_minmax"
      , "VK_EXT_shader_viewport_index_layer"
      , "VK_KHR_shared_presentable_image"
      , "VK_KHR_storage_buffer_storage_class"
      , "VK_KHR_surface"
      , "VK_EXT_swapchain_colorspace"
      , "VK_KHR_swapchain"
      , "VK_AMD_texture_gather_bias_lod"
      , "VK_EXT_validation_cache"
      , "VK_EXT_validation_flags"
      , "VK_EXT_vertex_attribute_divisor"
      , "VK_NN_vi_surface"
      , "VK_NV_viewport_array2"
      , "VK_NV_viewport_swizzle"
      ]

let Win32Specific =
      [ "VK_KHR_external_fence_win32"
      , "VK_KHR_external_memory_win32"
      , "VK_NV_external_memory_win32"
      , "VK_KHR_external_semaphore_win32"
      , "VK_EXT_full_screen_exclusive"
      , "VK_KHR_win32_keyed_mutex"
      , "VK_NV_win32_keyed_mutex"
      , "VK_KHR_win32_surface"
      , "VK_KHR_surface"
      , "VK_NV_external_memory"
      , "VK_NV_external_memory_capabilities"
      , "VK_KHR_get_surface_capabilities2"
      , "VK_KHR_swapchain"
      , "Implements"
      ]

let UnixSpecific =
      [ "VK_EXT_acquire_xlib_display"
      , "VK_KHR_android_surface"
      , "VK_KHR_external_fence_fd"
      , "VK_EXT_external_memory_dma_buf"
      , "VK_KHR_external_memory_fd"
      , "VK_KHR_external_semaphore_fd"
      , "VK_KHR_wayland_surface"
      , "VK_KHR_xcb_surface"
      , "VK_KHR_xlib_surface"
      , "VK_KHR_surface"
      , "VK_KHR_display"
      , "Implements"
      ]

let MacSpecific =
      [ "VK_MVK_ios_surface"
      , "VK_MVK_macos_surface"
      , "VK_KHR_surface"
      , "Implements"
      ]

in  { PlatformIndependent, Win32Specific, UnixSpecific, MacSpecific }
