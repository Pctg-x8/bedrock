use std::fmt::Write;

pub struct EntryPointFunctionPointerTableInputFormatter<'t>(&'t [(&'static str, &'static str)]);
impl<'t> core::fmt::Display for EntryPointFunctionPointerTableInputFormatter<'t> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut has_prev = false;
        for (_, t) in self.0 {
            if has_prev {
                f.write_str(", ")?;
            }

            t.fmt(f)?;
            has_prev = true;
        }

        Ok(())
    }
}

pub struct EntrypointFunctionInputFormatter<'t>(pub &'t [(&'static str, &'static str)]);
impl core::fmt::Display for EntrypointFunctionInputFormatter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut has_prev = false;
        for (n, t) in self.0 {
            if has_prev {
                f.write_str(", ")?;
            }

            write!(f, "{n}: {t}")?;
            has_prev = true;
        }

        Ok(())
    }
}

pub struct EntrypointFunctionInputForwardingFormatter<'t>(pub &'t [(&'static str, &'static str)]);
impl core::fmt::Display for EntrypointFunctionInputForwardingFormatter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut has_prev = false;
        for (n, _) in self.0 {
            if has_prev {
                f.write_str(", ")?;
            }

            f.write_str(n)?;
            has_prev = true;
        }

        Ok(())
    }
}

pub struct AllFeatureGatePrinter<'x>(&'x [&'x str]);
impl core::fmt::Display for AllFeatureGatePrinter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.len() {
            0 => unreachable!("no feature gated"),
            1 => write!(f, r#"#[cfg(feature = "{}")]"#, self.0[0]),
            _ => {
                f.write_str("#[cfg(all(")?;
                let mut has_prev = false;
                for n in self.0 {
                    if has_prev {
                        f.write_str(", ")?;
                    }

                    write!(f, r#"feature = "{n}""#)?;
                    has_prev = true;
                }
                f.write_str("))]")
            }
        }
    }
}

pub struct Entrypoint {
    pub export_name: &'static str,
    pub canonical_name: Option<&'static str>,
    pub ext_suffix: Option<&'static str>,
    pub inputs: &'static [(&'static str, &'static str)],
    pub output: Option<&'static str>,
    pub extensions: &'static [&'static str],
    pub promoted: Option<(u32, u32)>,
}
impl Entrypoint {
    pub const fn new(export_name: &'static str, inputs: &'static [(&'static str, &'static str)]) -> Self {
        Self {
            export_name,
            canonical_name: None,
            ext_suffix: None,
            inputs,
            output: None,
            extensions: &[],
            promoted: None,
        }
    }

    pub const fn canonical_name(self, name: &'static str) -> Self {
        Self {
            canonical_name: Some(name),
            ..self
        }
    }

    pub const fn with_suffix(self, suffix: &'static str) -> Self {
        Self {
            ext_suffix: Some(suffix),
            ..self
        }
    }

    pub const fn with_return(self, rt: &'static str) -> Self {
        Self {
            output: Some(rt),
            ..self
        }
    }

    pub const fn with_result(self) -> Self {
        Self {
            output: Some("VkResult"),
            ..self
        }
    }

    pub const fn for_extensions(self, exts: &'static [&'static str], suffix: &'static str) -> Self {
        Self {
            extensions: exts,
            ext_suffix: Some(suffix),
            ..self
        }
    }

    pub const fn promoted_at(self, major: u32, minor: u32) -> Self {
        Self {
            promoted: Some((major, minor)),
            ..self
        }
    }

    pub fn gen_fptbl_entry(&self, sink: &mut (impl core::fmt::Write + ?Sized)) -> core::fmt::Result {
        if !self.extensions.is_empty() {
            writeln!(sink, r#"    {}"#, AllFeatureGatePrinter(self.extensions))?;
        }
        if let Some((major, minor)) = self.promoted {
            writeln!(sink, r#"    #[cfg(feature = "Allow{major}_{minor}APIs")]"#)?;
        }

        write!(sink, r#"    {}: {}"#, ExportNameWriter(self), PFNNameWriter(self))
    }

    pub fn gen_fptbl_init_entry(&self, sink: &mut (impl core::fmt::Write + ?Sized)) -> core::fmt::Result {
        if !self.extensions.is_empty() {
            writeln!(sink, r#"        {}"#, AllFeatureGatePrinter(self.extensions))?;
        }
        if let Some((major, minor)) = self.promoted {
            writeln!(sink, r#"        #[cfg(feature = "Allow{major}_{minor}APIs")]"#)?;
        }

        write!(
            sink,
            r#"        {}: {}({})"#,
            self.export_name,
            PFNNameWriter(self),
            StubNameWriter(self)
        )
    }

    pub fn gen_stub(&self, sink: &mut (impl core::fmt::Write + ?Sized)) -> core::fmt::Result {
        if !self.extensions.is_empty() {
            writeln!(sink, r#"{}"#, AllFeatureGatePrinter(self.extensions))?;
        }
        if let Some((major, minor)) = self.promoted {
            writeln!(sink, r#"#[cfg(feature = "Allow{major}_{minor}APIs")]"#)?;
        }

        write!(
            sink,
            r#"#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
unsafe extern "system" fn {}({})"#,
            StubNameWriter(self),
            EntrypointFunctionInputFormatter(self.inputs)
        )?;
        if let Some(ref o) = self.output {
            write!(sink, " -> {o}")?;
        }
        sink.write_str(" {\n")?;

        writeln!(
            sink,
            r#"    let fp: {pfn} = crate::resolver::get_resolver().load_function_unconstrainted(<{pfn}>::NAME_CSTR);"#,
            pfn = PFNNameWriter(self)
        )?;
        writeln!(sink, r#"    FPTBL.{} = fp;"#, ExportNameWriter(self))?;
        writeln!(
            sink,
            r#"    (fp.0)({})"#,
            EntrypointFunctionInputForwardingFormatter(self.inputs)
        )?;

        sink.write_str("}\n")?;
        Ok(())
    }

    pub fn gen_entrypoint(&self, sink: &mut (impl core::fmt::Write + ?Sized)) -> core::fmt::Result {
        if !self.extensions.is_empty() {
            writeln!(sink, r#"{}"#, AllFeatureGatePrinter(self.extensions))?;
        }
        if let Some((major, minor)) = self.promoted {
            writeln!(sink, r#"#[cfg(feature = "Allow{major}_{minor}APIs")]"#)?;
        }

        write!(
            sink,
            r#"#[rustfmt::skip] #[inline(always)]
pub unsafe fn {}({})"#,
            ExportNameWriter(self),
            EntrypointFunctionInputFormatter(self.inputs)
        )?;
        if let Some(ref o) = self.output {
            write!(sink, " -> {o}")?;
        }
        sink.write_str(" {\n")?;

        writeln!(
            sink,
            r#"    #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))] {{ (FPTBL.{}.0)({}) }}"#,
            ExportNameWriter(self),
            EntrypointFunctionInputForwardingFormatter(self.inputs)
        )?;
        writeln!(
            sink,
            r#"    #[cfg(not(any(feature = "DynamicLoaded", feature = "CustomResolver")))] {{ {}({}) }}"#,
            CanonicalNameWriter(self),
            EntrypointFunctionInputForwardingFormatter(self.inputs)
        )?;

        sink.write_str("}\n")?;
        Ok(())
    }
}

pub struct CanonicalNameWriter<'e>(pub &'e Entrypoint);
impl core::fmt::Display for CanonicalNameWriter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(n) = self.0.canonical_name {
            return f.write_str(n);
        }

        f.write_str("vk")?;
        let mut capitalize = true;
        for c in self.0.export_name.chars() {
            if capitalize {
                f.write_char(c.to_ascii_uppercase())?;
            } else if c != '_' {
                f.write_char(c)?;
            }

            capitalize = c == '_';
        }

        if let Some(ext) = self.0.ext_suffix {
            f.write_str(ext)?;
        }

        Ok(())
    }
}

pub struct PFNNameWriter<'e>(pub &'e Entrypoint);
impl core::fmt::Display for PFNNameWriter<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PFN_{}", CanonicalNameWriter(self.0))
    }
}

pub struct StubNameWriter<'e>(pub &'e Entrypoint);
impl core::fmt::Display for StubNameWriter<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "stub_{}", self.0.export_name)?;
        if let Some(ext) = self.0.ext_suffix {
            write!(f, "_{}", ext.to_lowercase())?;
        }

        Ok(())
    }
}

pub struct ExportNameWriter<'e>(pub &'e Entrypoint);
impl core::fmt::Display for ExportNameWriter<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.export_name)?;
        if let Some(ext) = self.0.ext_suffix {
            write!(f, "_{}", ext.to_lowercase())?;
        }

        Ok(())
    }
}

const CATALOG: &'static [Entrypoint] = &[
    Entrypoint::new(
        "create_instance",
        &[
            ("create_info", "*const VkInstanceCreateInfo"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("instance_out", "*mut VkInstance"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "destroy_instance",
        &[
            ("instance", "VkInstance"),
            ("allocator", "*const VkAllocationCallbacks"),
        ],
    ),
    Entrypoint::new(
        "enumerate_physical_devices",
        &[
            ("instance", "VkInstance"),
            ("physical_devices_count_out", "*mut u32"),
            ("physical_devices_out", "*mut VkPhysicalDevice"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "get_physical_device_features",
        &[
            ("physical_device", "VkPhysicalDevice"),
            ("features_out", "*mut VkPhysicalDeviceFeatures"),
        ],
    ),
    Entrypoint::new(
        "get_physical_device_format_properties",
        &[
            ("physical_device", "VkPhysicalDevice"),
            ("format", "VkFormat"),
            ("format_properties_out", "*mut VkFormatProperties"),
        ],
    ),
    Entrypoint::new(
        "get_physical_device_image_format_properties",
        &[
            ("physical_device", "VkPhysicalDevice"),
            ("format", "VkFormat"),
            ("image_type", "VkImageType"),
            ("tiling", "VkImageTiling"),
            ("usage", "VkImageUsageFlags"),
            ("flags", "VkImageCreateFlags"),
            ("image_format_properties_out", "*mut VkImageFormatProperties"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "get_physical_device_properties",
        &[
            ("physical_device", "VkPhysicalDevice"),
            ("properties_out", "*mut VkPhysicalDeviceProperties"),
        ],
    ),
    Entrypoint::new(
        "get_physical_device_queue_family_properties",
        &[
            ("physical_device", "VkPhysicalDevice"),
            ("queue_family_properties_count_out", "*mut u32"),
            ("queue_family_properties_out", "*mut VkQueueFamilyProperties"),
        ],
    ),
    Entrypoint::new(
        "get_physical_device_memory_properties",
        &[
            ("physical_device", "VkPhysicalDevice"),
            ("memory_properties_out", "*mut VkPhysicalDeviceMemoryProperties"),
        ],
    ),
    Entrypoint::new(
        "get_instance_proc_addr",
        &[("instance", "VkInstance"), ("name", "*const c_char")],
    )
    .with_return("Option<PFN_vkVoidFunction>"),
    Entrypoint::new(
        "get_device_proc_addr",
        &[("device", "VkDevice"), ("name", "*const c_char")],
    )
    .with_return("Option<PFN_vkVoidFunction>"),
    Entrypoint::new(
        "create_device",
        &[
            ("physical_device", "VkPhysicalDevice"),
            ("create_info", "*const VkDeviceCreateInfo"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("device_out", "*mut VkDevice"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "destroy_device",
        &[("device", "VkDevice"), ("allocator", "*const VkAllocationCallbacks")],
    ),
    Entrypoint::new(
        "enumerate_instance_extension_properties",
        &[
            ("layer_name", "*const c_char"),
            ("property_count_out", "*mut u32"),
            ("properties_out", "*mut VkExtensionProperties"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "enumerate_device_extension_properties",
        &[
            ("physical_device", "VkPhysicalDevice"),
            ("layer_name", "*const c_char"),
            ("property_count_out", "*mut u32"),
            ("properties_out", "*mut VkExtensionProperties"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "enumerate_instance_layer_properties",
        &[
            ("property_count_out", "*mut u32"),
            ("properties_out", "*mut VkLayerProperties"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "enumerate_device_layer_properties",
        &[
            ("physical_device", "VkPhysicalDevice"),
            ("property_count_out", "*mut u32"),
            ("properties_out", "*mut VkLayerProperties"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "get_device_queue",
        &[
            ("device", "VkDevice"),
            ("queue_family_index", "u32"),
            ("queue_index", "u32"),
            ("queue_out", "*mut VkQueue"),
        ],
    ),
    Entrypoint::new(
        "queue_submit",
        &[
            ("queue", "VkQueue"),
            ("submit_count", "u32"),
            ("submits", "*const VkSubmitInfo"),
            ("fence", "VkFence"),
        ],
    )
    .with_result(),
    Entrypoint::new("queue_wait_idle", &[("queue", "VkQueue")]).with_result(),
    Entrypoint::new("device_wait_idle", &[("device", "VkDevice")]).with_result(),
    Entrypoint::new(
        "allocate_memory",
        &[
            ("device", "VkDevice"),
            ("allocate_info", "*const VkMemoryAllocateInfo"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("memory_out", "*mut VkDeviceMemory"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "free_memory",
        &[
            ("device", "VkDevice"),
            ("memory", "VkDeviceMemory"),
            ("allocator", "*const VkAllocationCallbacks"),
        ],
    ),
    Entrypoint::new(
        "map_memory",
        &[
            ("device", "VkDevice"),
            ("memory", "VkDeviceMemory"),
            ("offset", "VkDeviceSize"),
            ("size", "VkDeviceSize"),
            ("flags", "VkMemoryMapFlags"),
            ("data_ptr_out", "*mut *mut c_void"),
        ],
    )
    .with_result(),
    Entrypoint::new("unmap_memory", &[("device", "VkDevice"), ("memory", "VkDeviceMemory")]),
    Entrypoint::new(
        "flush_mapped_memory_ranges",
        &[
            ("device", "VkDevice"),
            ("memory_range_count", "u32"),
            ("memory_ranges", "*const VkMappedMemoryRange"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "invalidate_mapped_memory_ranges",
        &[
            ("device", "VkDevice"),
            ("memory_range_count", "u32"),
            ("memory_ranges", "*const VkMappedMemoryRange"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "get_device_memory_commitment",
        &[
            ("device", "VkDevice"),
            ("memory", "VkDeviceMemory"),
            ("committed_memory_bytes_out", "*mut VkDeviceSize"),
        ],
    ),
    Entrypoint::new(
        "bind_buffer_memory",
        &[
            ("device", "VkDevice"),
            ("buffer", "VkBuffer"),
            ("memory", "VkDeviceMemory"),
            ("memory_offset", "VkDeviceSize"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "bind_image_memory",
        &[
            ("device", "VkDevice"),
            ("image", "VkImage"),
            ("memory", "VkDeviceMemory"),
            ("memory_offset", "VkDeviceSize"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "get_buffer_memory_requirements",
        &[
            ("device", "VkDevice"),
            ("buffer", "VkBuffer"),
            ("memory_requirements_out", "*mut VkMemoryRequirements"),
        ],
    ),
    Entrypoint::new(
        "get_image_memory_requirements",
        &[
            ("device", "VkDevice"),
            ("image", "VkImage"),
            ("memory_requirements_out", "*mut VkMemoryRequirements"),
        ],
    ),
    Entrypoint::new(
        "get_image_sparse_memory_requirements",
        &[
            ("device", "VkDevice"),
            ("image", "VkImage"),
            ("sparse_memory_requirement_count_out", "*mut u32"),
            ("sparse_memory_requirements_out", "*mut VkSparseImageMemoryRequirements"),
        ],
    ),
    Entrypoint::new(
        "get_physical_device_sparse_image_format_properties",
        &[
            ("physical_device", "VkPhysicalDevice"),
            ("format", "VkFormat"),
            ("r#type", "VkImageType"),
            ("samples", "VkSampleCountFlags"),
            ("usage", "VkImageUsageFlags"),
            ("tiling", "VkImageTiling"),
            ("property_count_out", "*mut u32"),
            ("properties_out", "*mut VkSparseImageFormatProperties"),
        ],
    ),
    Entrypoint::new(
        "queue_bind_sparse",
        &[
            ("queue", "VkQueue"),
            ("bind_info_count", "u32"),
            ("bind_info", "*const VkBindSparseInfo"),
            ("fence", "VkFence"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "create_fence",
        &[
            ("device", "VkDevice"),
            ("create_info", "*const VkFenceCreateInfo"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("fence_out", "*mut VkFence"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "destroy_fence",
        &[
            ("device", "VkDevice"),
            ("fence", "VkFence"),
            ("allocator", "*const VkAllocationCallbacks"),
        ],
    ),
    Entrypoint::new(
        "reset_fences",
        &[
            ("device", "VkDevice"),
            ("fence_count", "u32"),
            ("fences", "*const VkFence"),
        ],
    )
    .with_result(),
    Entrypoint::new("get_fence_status", &[("device", "VkDevice"), ("fence", "VkFence")]).with_result(),
    Entrypoint::new(
        "wait_for_fences",
        &[
            ("device", "VkDevice"),
            ("fence_count", "u32"),
            ("fences", "*const VkFence"),
            ("wait_all", "VkBool32"),
            ("timeout", "u64"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "create_semaphore",
        &[
            ("device", "VkDevice"),
            ("create_info", "*const VkSemaphoreCreateInfo"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("semaphore_out", "*mut VkSemaphore"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "destroy_semaphore",
        &[
            ("device", "VkDevice"),
            ("semaphore", "VkSemaphore"),
            ("allocator", "*const VkAllocationCallbacks"),
        ],
    ),
    Entrypoint::new(
        "create_event",
        &[
            ("device", "VkDevice"),
            ("create_info", "*const VkEventCreateInfo"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("event_out", "*mut VkEvent"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "destroy_event",
        &[
            ("device", "VkDevice"),
            ("event", "VkEvent"),
            ("allocator", "*const VkAllocationCallbacks"),
        ],
    ),
    Entrypoint::new("get_event_status", &[("device", "VkDevice"), ("event", "VkEvent")]).with_result(),
    Entrypoint::new("set_event", &[("device", "VkDevice"), ("event", "VkEvent")]).with_result(),
    Entrypoint::new("reset_event", &[("device", "VkDevice"), ("event", "VkEvent")]).with_result(),
    Entrypoint::new(
        "create_query_pool",
        &[
            ("device", "VkDevice"),
            ("create_info", "*const VkQueryPoolCreateInfo"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("query_pool_out", "*mut VkQueryPool"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "destroy_query_pool",
        &[
            ("device", "VkDevice"),
            ("query_pool", "VkQueryPool"),
            ("allocator", "*const VkAllocationCallbacks"),
        ],
    ),
    Entrypoint::new(
        "get_query_pool_results",
        &[
            ("device", "VkDevice"),
            ("query_pool", "VkQueryPool"),
            ("first_query", "u32"),
            ("query_count", "u32"),
            ("data_size", "usize"),
            ("data_out", "*mut c_void"),
            ("stride", "VkDeviceSize"),
            ("flags", "VkQueryResultFlags"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "create_buffer",
        &[
            ("device", "VkDevice"),
            ("create_info", "*const VkBufferCreateInfo"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("buffer_out", "*mut VkBuffer"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "destroy_buffer",
        &[
            ("device", "VkDevice"),
            ("buffer", "VkBuffer"),
            ("allocator", "*const VkAllocationCallbacks"),
        ],
    ),
    Entrypoint::new(
        "create_buffer_view",
        &[
            ("device", "VkDevice"),
            ("create_info", "*const VkBufferViewCreateInfo"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("view_out", "*mut VkBufferView"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "destroy_buffer_view",
        &[
            ("device", "VkDevice"),
            ("buffer_view", "VkBufferView"),
            ("allocator", "*const VkAllocationCallbacks"),
        ],
    ),
    Entrypoint::new(
        "create_image",
        &[
            ("device", "VkDevice"),
            ("create_info", "*const VkImageCreateInfo"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("image_out", "*mut VkImage"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "destroy_image",
        &[
            ("device", "VkDevice"),
            ("image", "VkImage"),
            ("allocator", "*const VkAllocationCallbacks"),
        ],
    ),
    Entrypoint::new(
        "get_image_subresource_layout",
        &[
            ("device", "VkDevice"),
            ("image", "VkImage"),
            ("subresource", "*const VkImageSubresource"),
            ("layout_out", "*mut VkSubresourceLayout"),
        ],
    ),
    Entrypoint::new(
        "create_image_view",
        &[
            ("device", "VkDevice"),
            ("create_info", "*const VkImageViewCreateInfo"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("view_out", "*mut VkImageView"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "destroy_image_view",
        &[
            ("device", "VkDevice"),
            ("image_view", "VkImageView"),
            ("allocator", "*const VkAllocationCallbacks"),
        ],
    ),
    Entrypoint::new(
        "create_shader_module",
        &[
            ("device", "VkDevice"),
            ("create_info", "*const VkShaderModuleCreateInfo"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("shader_module_out", "*mut VkShaderModule"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "destroy_shader_module",
        &[
            ("device", "VkDevice"),
            ("shader_module", "VkShaderModule"),
            ("allocator", "*const VkAllocationCallbacks"),
        ],
    ),
    Entrypoint::new(
        "create_pipeline_cache",
        &[
            ("device", "VkDevice"),
            ("create_info", "*const VkPipelineCacheCreateInfo"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("pipeline_cache_out", "*mut VkPipelineCache"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "destroy_pipeline_cache",
        &[
            ("device", "VkDevice"),
            ("pipeline_cache", "VkPipelineCache"),
            ("allocator", "*const VkAllocationCallbacks"),
        ],
    ),
    Entrypoint::new(
        "get_pipeline_cache_data",
        &[
            ("device", "VkDevice"),
            ("pipeline_cache", "VkPipelineCache"),
            ("data_size_out", "*mut usize"),
            ("data_out", "*mut c_void"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "merge_pipeline_caches",
        &[
            ("device", "VkDevice"),
            ("dst_cache", "VkPipelineCache"),
            ("src_cache_count", "u32"),
            ("src_caches", "*const VkPipelineCache"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "create_graphics_pipelines",
        &[
            ("device", "VkDevice"),
            ("pipeline_cache", "VkPipelineCache"),
            ("create_info_count", "u32"),
            ("create_infos", "*const VkGraphicsPipelineCreateInfo"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("pipelines_out", "*mut VkPipeline"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "create_compute_pipelines",
        &[
            ("device", "VkDevice"),
            ("pipeline_cache", "VkPipelineCache"),
            ("create_info_count", "u32"),
            ("create_infos", "*const VkComputePipelineCreateInfo"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("pipelines_out", "*mut VkPipeline"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "destroy_pipeline",
        &[
            ("device", "VkDevice"),
            ("pipeline", "VkPipeline"),
            ("allocator", "*const VkAllocationCallbacks"),
        ],
    ),
    Entrypoint::new(
        "create_pipeline_layout",
        &[
            ("device", "VkDevice"),
            ("create_info", "*const VkPipelineLayoutCreateInfo"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("pipeline_layout_out", "*mut VkPipelineLayout"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "destroy_pipeline_layout",
        &[
            ("device", "VkDevice"),
            ("pipeline_layout", "VkPipelineLayout"),
            ("allocator", "*const VkAllocationCallbacks"),
        ],
    ),
    Entrypoint::new(
        "create_sampler",
        &[
            ("device", "VkDevice"),
            ("create_info", "*const VkSamplerCreateInfo"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("sampler_out", "*mut VkSampler"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "destroy_sampler",
        &[
            ("device", "VkDevice"),
            ("sampler", "VkSampler"),
            ("allocator", "*const VkAllocationCallbacks"),
        ],
    ),
    Entrypoint::new(
        "create_descriptor_set_layout",
        &[
            ("device", "VkDevice"),
            ("create_info", "*const VkDescriptorSetLayoutCreateInfo"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("set_layout_out", "*mut VkDescriptorSetLayout"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "destroy_descriptor_set_layout",
        &[
            ("device", "VkDevice"),
            ("descriptor_set_layout", "VkDescriptorSetLayout"),
            ("allocator", "*const VkAllocationCallbacks"),
        ],
    ),
    Entrypoint::new(
        "create_descriptor_pool",
        &[
            ("device", "VkDevice"),
            ("create_info", "*const VkDescriptorPoolCreateInfo"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("descriptor_pool_out", "*mut VkDescriptorPool"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "destroy_descriptor_pool",
        &[
            ("device", "VkDevice"),
            ("descriptor_pool", "VkDescriptorPool"),
            ("allocator", "*const VkAllocationCallbacks"),
        ],
    ),
    Entrypoint::new(
        "reset_descriptor_pool",
        &[
            ("device", "VkDevice"),
            ("descriptor_pool", "VkDescriptorPool"),
            ("flags", "VkDescriptorPoolResetFlags"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "allocate_descriptor_sets",
        &[
            ("device", "VkDevice"),
            ("allocate_info", "*const VkDescriptorSetAllocateInfo"),
            ("descriptor_sets_out", "*mut VkDescriptorSet"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "free_descriptor_sets",
        &[
            ("device", "VkDevice"),
            ("descriptor_pool", "VkDescriptorPool"),
            ("descriptor_set_count", "u32"),
            ("descriptor_sets", "*const VkDescriptorSet"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "update_descriptor_sets",
        &[
            ("device", "VkDevice"),
            ("descriptor_write_count", "u32"),
            ("descriptor_writes", "*const VkWriteDescriptorSet"),
            ("descriptor_copy_count", "u32"),
            ("descriptor_copies", "*const VkCopyDescriptorSet"),
        ],
    ),
    Entrypoint::new(
        "create_framebuffer",
        &[
            ("device", "VkDevice"),
            ("create_info", "*const VkFramebufferCreateInfo"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("framebuffer_out", "*mut VkFramebuffer"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "destroy_framebuffer",
        &[
            ("device", "VkDevice"),
            ("framebuffer", "VkFramebuffer"),
            ("allocator", "*const VkAllocationCallbacks"),
        ],
    ),
    Entrypoint::new(
        "create_render_pass",
        &[
            ("device", "VkDevice"),
            ("create_info", "*const VkRenderPassCreateInfo"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("render_pass_out", "*mut VkRenderPass"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "destroy_render_pass",
        &[
            ("device", "VkDevice"),
            ("render_pass", "VkRenderPass"),
            ("allocator", "*const VkAllocationCallbacks"),
        ],
    ),
    Entrypoint::new(
        "get_render_area_granularity",
        &[
            ("device", "VkDevice"),
            ("render_pass", "VkRenderPass"),
            ("granularity_out", "*mut VkExtent2D"),
        ],
    ),
    Entrypoint::new(
        "create_command_pool",
        &[
            ("device", "VkDevice"),
            ("create_info", "*const VkCommandPoolCreateInfo"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("command_pool_out", "*mut VkCommandPool"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "destroy_command_pool",
        &[
            ("device", "VkDevice"),
            ("command_pool", "VkCommandPool"),
            ("allocator", "*const VkAllocationCallbacks"),
        ],
    ),
    Entrypoint::new(
        "reset_command_pool",
        &[
            ("device", "VkDevice"),
            ("command_pool", "VkCommandPool"),
            ("flags", "VkCommandPoolResetFlags"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "allocate_command_buffers",
        &[
            ("device", "VkDevice"),
            ("allocate_info", "*const VkCommandBufferAllocateInfo"),
            ("command_buffers_out", "*mut VkCommandBuffer"),
        ],
    )
    .with_result(),
    Entrypoint::new(
        "free_command_buffers",
        &[
            ("device", "VkDevice"),
            ("command_pool", "VkCommandPool"),
            ("command_buffer_count", "u32"),
            ("command_buffers", "*const VkCommandBuffer"),
        ],
    ),
    Entrypoint::new(
        "begin_command_buffer",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("begin_info", "*const VkCommandBufferBeginInfo"),
        ],
    )
    .with_result(),
    Entrypoint::new("end_command_buffer", &[("command_buffer", "VkCommandBuffer")]).with_result(),
    Entrypoint::new(
        "reset_command_buffer",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("flags", "VkCommandBufferResetFlags"),
        ],
    )
    .with_result(),
    // 1.0 commands
    Entrypoint::new(
        "cmd_bind_pipeline",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("pipeline_bind_point", "VkPipelineBindPoint"),
            ("pipeline", "VkPipeline"),
        ],
    ),
    Entrypoint::new(
        "cmd_set_viewport",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("first_viewport", "u32"),
            ("viewport_count", "u32"),
            ("viewports", "*const VkViewport"),
        ],
    ),
    Entrypoint::new(
        "cmd_set_scissor",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("first_scissor", "u32"),
            ("scissor_count", "u32"),
            ("scissors", "*const VkRect2D"),
        ],
    ),
    Entrypoint::new(
        "cmd_set_line_width",
        &[("command_buffer", "VkCommandBuffer"), ("line_width", "c_float")],
    ),
    Entrypoint::new(
        "cmd_set_depth_bias",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("depth_bias_constant_factor", "c_float"),
            ("depth_bias_clamp", "c_float"),
            ("depth_bias_slope_factor", "c_float"),
        ],
    ),
    Entrypoint::new(
        "cmd_set_blend_constants",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("blend_constants", "*const c_float"),
        ],
    ),
    Entrypoint::new(
        "cmd_set_depth_bounds",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("min_depth_bounds", "c_float"),
            ("max_depth_bounds", "c_float"),
        ],
    ),
    Entrypoint::new(
        "cmd_set_stencil_compare_mask",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("face_mask", "VkStencilFaceFlags"),
            ("compare_mask", "u32"),
        ],
    ),
    Entrypoint::new(
        "cmd_set_stencil_write_mask",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("face_mask", "VkStencilFaceFlags"),
            ("write_mask", "u32"),
        ],
    ),
    Entrypoint::new(
        "cmd_set_stencil_reference",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("face_mask", "VkStencilFaceFlags"),
            ("reference", "u32"),
        ],
    ),
    Entrypoint::new(
        "cmd_bind_descriptor_sets",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("pipeline_bind_point", "VkPipelineBindPoint"),
            ("layout", "VkPipelineLayout"),
            ("first_set", "u32"),
            ("descriptor_set_count", "u32"),
            ("descriptor_sets", "*const VkDescriptorSet"),
            ("dynamic_offset_count", "u32"),
            ("dynamic_offsets", "*const u32"),
        ],
    ),
    Entrypoint::new(
        "cmd_bind_index_buffer",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("buffer", "VkBuffer"),
            ("offset", "VkDeviceSize"),
            ("index_type", "VkIndexType"),
        ],
    ),
    Entrypoint::new(
        "cmd_bind_vertex_buffers",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("first_binding", "u32"),
            ("binding_count", "u32"),
            ("buffers", "*const VkBuffer"),
            ("offsets", "*const VkDeviceSize"),
        ],
    ),
    Entrypoint::new(
        "cmd_draw",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("vertex_count", "u32"),
            ("instance_count", "u32"),
            ("first_vertex", "u32"),
            ("first_index", "u32"),
        ],
    ),
    Entrypoint::new(
        "cmd_draw_indexed",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("index_count", "u32"),
            ("instance_count", "u32"),
            ("first_index", "u32"),
            ("vertex_offset", "i32"),
            ("first_instance", "u32"),
        ],
    ),
    Entrypoint::new(
        "cmd_draw_indirect",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("buffer", "VkBuffer"),
            ("offset", "VkDeviceSize"),
            ("draw_count", "u32"),
            ("stride", "u32"),
        ],
    ),
    Entrypoint::new(
        "cmd_draw_indexed_indirect",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("buffer", "VkBuffer"),
            ("offset", "VkDeviceSize"),
            ("draw_count", "u32"),
            ("stride", "u32"),
        ],
    ),
    Entrypoint::new(
        "cmd_dispatch",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("group_count_x", "u32"),
            ("group_count_y", "u32"),
            ("group_count_z", "u32"),
        ],
    ),
    Entrypoint::new(
        "cmd_dispatch_indirect",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("buffer", "VkBuffer"),
            ("offset", "VkDeviceSize"),
        ],
    ),
    Entrypoint::new(
        "cmd_copy_buffer",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("src_buffer", "VkBuffer"),
            ("dst_buffer", "VkBuffer"),
            ("region_count", "u32"),
            ("regions", "*const VkBufferCopy"),
        ],
    ),
    Entrypoint::new(
        "cmd_copy_image",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("src_image", "VkImage"),
            ("src_image_layout", "VkImageLayout"),
            ("dst_image", "VkImage"),
            ("dst_image_layout", "VkImageLayout"),
            ("region_count", "u32"),
            ("regions", "*const VkImageCopy"),
        ],
    ),
    Entrypoint::new(
        "cmd_blit_image",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("src_image", "VkImage"),
            ("src_image_layout", "VkImageLayout"),
            ("dst_image", "VkImage"),
            ("dst_image_layout", "VkImageLayout"),
            ("region_count", "u32"),
            ("regions", "*const VkImageBlit"),
            ("filter", "VkFilter"),
        ],
    ),
    Entrypoint::new(
        "cmd_copy_buffer_to_image",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("src_buffer", "VkBuffer"),
            ("dst_image", "VkImage"),
            ("dst_image_layout", "VkImageLayout"),
            ("region_count", "u32"),
            ("regions", "*const VkBufferImageCopy"),
        ],
    ),
    Entrypoint::new(
        "cmd_copy_image_to_buffer",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("src_image", "VkImage"),
            ("src_image_layout", "VkImageLayout"),
            ("dst_buffer", "VkBuffer"),
            ("region_count", "u32"),
            ("regions", "*const VkBufferImageCopy"),
        ],
    ),
    Entrypoint::new(
        "cmd_update_buffer",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("dst_buffer", "VkBuffer"),
            ("dst_offset", "VkDeviceSize"),
            ("data_size", "VkDeviceSize"),
            ("data", "*const c_void"),
        ],
    ),
    Entrypoint::new(
        "cmd_fill_buffer",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("dst_buffer", "VkBuffer"),
            ("dst_offset", "VkDeviceSize"),
            ("size", "VkDeviceSize"),
            ("data", "u32"),
        ],
    ),
    Entrypoint::new(
        "cmd_clear_color_image",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("image", "VkImage"),
            ("image_layout", "VkImageLayout"),
            ("color", "*const VkClearColorValue"),
            ("range_count", "u32"),
            ("ranges", "*const VkImageSubresourceRange"),
        ],
    ),
    Entrypoint::new(
        "cmd_clear_depth_stencil_image",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("image", "VkImage"),
            ("image_layout", "VkImageLayout"),
            ("depth_stencil", "*const VkClearDepthStencilValue"),
            ("range_count", "u32"),
            ("ranges", "*const VkImageSubresourceRange"),
        ],
    ),
    Entrypoint::new(
        "cmd_clear_attachments",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("attachment_count", "u32"),
            ("attachments", "*const VkClearAttachment"),
            ("rect_count", "u32"),
            ("rects", "*const VkClearRect"),
        ],
    ),
    Entrypoint::new(
        "cmd_resolve_image",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("src_image", "VkImage"),
            ("src_image_layout", "VkImageLayout"),
            ("dst_image", "VkImage"),
            ("dst_image_layout", "VkImageLayout"),
            ("region_count", "u32"),
            ("regions", "*const VkImageResolve"),
        ],
    ),
    Entrypoint::new(
        "cmd_set_event",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("event", "VkEvent"),
            ("stage_mask", "VkPipelineStageFlags"),
        ],
    ),
    Entrypoint::new(
        "cmd_reset_event",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("event", "VkEvent"),
            ("stage_mask", "VkPipelineStageFlags"),
        ],
    ),
    Entrypoint::new(
        "cmd_wait_events",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("event_count", "u32"),
            ("events", "*const VkEvent"),
            ("src_stage_mask", "VkPipelineStageFlags"),
            ("dst_stage_mask", "VkPipelineStageFlags"),
            ("memory_barrier_count", "u32"),
            ("memory_barriers", "*const VkMemoryBarrier"),
            ("buffer_memory_barrier_count", "u32"),
            ("buffer_memory_barriers", "*const VkBufferMemoryBarrier"),
            ("image_memory_barrier_count", "u32"),
            ("image_memory_barriers", "*const VkImageMemoryBarrier"),
        ],
    ),
    Entrypoint::new(
        "cmd_pipeline_barrier",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("src_stage_mask", "VkPipelineStageFlags"),
            ("dst_stage_mask", "VkPipelineStageFlags"),
            ("dependency_flags", "VkDependencyFlags"),
            ("memory_barrier_count", "u32"),
            ("memory_barriers", "*const VkMemoryBarrier"),
            ("buffer_memory_barrier_count", "u32"),
            ("buffer_memory_barriers", "*const VkBufferMemoryBarrier"),
            ("image_memory_barrier_count", "u32"),
            ("image_memory_barriers", "*const VkImageMemoryBarrier"),
        ],
    ),
    Entrypoint::new(
        "cmd_begin_query",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("query_pool", "VkQueryPool"),
            ("query", "u32"),
            ("flags", "VkQueryControlFlags"),
        ],
    ),
    Entrypoint::new(
        "cmd_end_query",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("query_pool", "VkQueryPool"),
            ("query", "u32"),
        ],
    ),
    Entrypoint::new(
        "cmd_reset_query_pool",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("query_pool", "VkQueryPool"),
            ("first_query", "u32"),
            ("query_count", "u32"),
        ],
    ),
    Entrypoint::new(
        "cmd_write_timestamp",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("pipeline_stage", "VkPipelineStageFlags"),
            ("query_pool", "VkQueryPool"),
            ("query", "u32"),
        ],
    ),
    Entrypoint::new(
        "cmd_copy_query_pool_results",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("query_pool", "VkQueryPool"),
            ("first_query", "u32"),
            ("query_count", "u32"),
            ("dst_buffer", "VkBuffer"),
            ("dst_offset", "VkDeviceSize"),
            ("stride", "VkDeviceSize"),
            ("flags", "VkQueryResultFlags"),
        ],
    ),
    Entrypoint::new(
        "cmd_push_constants",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("layout", "VkPipelineLayout"),
            ("stage_flags", "VkShaderStageFlags"),
            ("offset", "u32"),
            ("size", "u32"),
            ("values", "*const c_void"),
        ],
    ),
    Entrypoint::new(
        "cmd_begin_render_pass",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("render_pass_begin_info", "*const VkRenderPassBeginInfo"),
            ("contents", "VkSubpassContents"),
        ],
    ),
    Entrypoint::new(
        "cmd_next_subpass",
        &[("command_buffer", "VkCommandBuffer"), ("contents", "VkSubpassContents")],
    ),
    Entrypoint::new("cmd_end_render_pass", &[("command_buffer", "VkCommandBuffer")]),
    Entrypoint::new(
        "cmd_execute_commands",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("command_buffer_count", "u32"),
            ("command_buffers", "*const VkCommandBuffer"),
        ],
    ),
    // 1.1 promoted
    Entrypoint::new("enumerate_instance_version", &[("api_version", "*mut u32")])
        .with_result()
        .promoted_at(1, 1),
    // surface extension
    Entrypoint::new(
        "destroy_surface",
        &[
            ("instance", "VkInstance"),
            ("surface", "VkSurfaceKHR"),
            ("allocator", "*const VkAllocationCallbacks"),
        ],
    )
    .for_extensions(&["VK_KHR_surface"], "KHR"),
    Entrypoint::new(
        "get_physical_device_surface_support",
        &[
            ("physical_device", "VkPhysicalDevice"),
            ("queue_family_index", "u32"),
            ("surface", "VkSurfaceKHR"),
            ("supported_out", "*mut VkBool32"),
        ],
    )
    .with_result()
    .for_extensions(&["VK_KHR_surface"], "KHR"),
    Entrypoint::new(
        "get_physical_device_surface_capabilities",
        &[
            ("physical_device", "VkPhysicalDevice"),
            ("surface", "VkSurfaceKHR"),
            ("surface_capabilities_out", "*mut VkSurfaceCapabilitiesKHR"),
        ],
    )
    .with_result()
    .for_extensions(&["VK_KHR_surface"], "KHR"),
    Entrypoint::new(
        "get_physical_device_surface_formats",
        &[
            ("physical_device", "VkPhysicalDevice"),
            ("surface", "VkSurfaceKHR"),
            ("surface_format_count_out", "*mut u32"),
            ("surface_formats_out", "*mut VkSurfaceFormatKHR"),
        ],
    )
    .with_result()
    .for_extensions(&["VK_KHR_surface"], "KHR"),
    Entrypoint::new(
        "get_physical_device_surface_present_modes",
        &[
            ("physical_device", "VkPhysicalDevice"),
            ("surface", "VkSurfaceKHR"),
            ("present_mode_count_out", "*mut u32"),
            ("present_modes_out", "*mut VkPresentModeKHR"),
        ],
    )
    .with_result()
    .for_extensions(&["VK_KHR_surface"], "KHR"),
    // swapchain extension
    Entrypoint::new(
        "create_swapchain",
        &[
            ("device", "VkDevice"),
            ("create_info", "*const VkSwapchainCreateInfoKHR"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("swapchain_out", "*mut VkSwapchainKHR"),
        ],
    )
    .with_result()
    .for_extensions(&["VK_KHR_swapchain"], "KHR"),
    Entrypoint::new(
        "destroy_swapchain",
        &[
            ("device", "VkDevice"),
            ("swapchain", "VkSwapchainKHR"),
            ("allocator", "*const VkAllocationCallbacks"),
        ],
    )
    .for_extensions(&["VK_KHR_swapchain"], "KHR"),
    Entrypoint::new(
        "get_swapchain_images",
        &[
            ("device", "VkDevice"),
            ("swapchain", "VkSwapchainKHR"),
            ("swapchain_image_count_out", "*mut u32"),
            ("swapchain_images_out", "*mut VkImage"),
        ],
    )
    .with_result()
    .for_extensions(&["VK_KHR_swapchain"], "KHR"),
    Entrypoint::new(
        "acquire_next_image",
        &[
            ("device", "VkDevice"),
            ("swapchain", "VkSwapchainKHR"),
            ("timeout", "u64"),
            ("semaphore", "VkSemaphore"),
            ("fence", "VkFence"),
            ("image_index_out", "*mut u32"),
        ],
    )
    .with_result()
    .for_extensions(&["VK_KHR_swapchain"], "KHR"),
    Entrypoint::new(
        "queue_present",
        &[("queue", "VkQueue"), ("present_info", "*const VkPresentInfoKHR")],
    )
    .with_result()
    .for_extensions(&["VK_KHR_swapchain"], "KHR"),
    // platform surface extension: xlib
    Entrypoint::new(
        "create_xlib_surface",
        &[
            ("instance", "VkInstance"),
            ("create_info", "*const VkXlibSurfaceCreateInfoKHR"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("surface_out", "*mut VkSurfaceKHR"),
        ],
    )
    .with_result()
    .for_extensions(&["VK_KHR_xlib_surface"], "KHR"),
    Entrypoint::new(
        "get_physical_device_xlib_presentation_support",
        &[
            ("physical_device", "VkPhysicalDevice"),
            ("queue_family_index", "u32"),
            ("dpy", "*mut x11::xlib::Display"),
            ("visual_id", "x11::xlib::VisualID"),
        ],
    )
    .with_return("VkBool32")
    .for_extensions(&["VK_KHR_xlib_surface"], "KHR"),
    // platform surface extension: xcb
    Entrypoint::new(
        "create_xcb_surface",
        &[
            ("instance", "VkInstance"),
            ("create_info", "*const VkXcbSurfaceCreateInfoKHR"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("surface_out", "*mut VkSurfaceKHR"),
        ],
    )
    .with_result()
    .for_extensions(&["VK_KHR_xcb_surface"], "KHR"),
    Entrypoint::new(
        "get_physical_device_xcb_presentation_support",
        &[
            ("physical_device", "VkPhysicalDevice"),
            ("queue_family_index", "u32"),
            ("connection", "*mut xcb::ffi::xcb_connection_t"),
            ("visual_id", "xcb::x::Visualid"),
        ],
    )
    .with_return("VkBool32")
    .for_extensions(&["VK_KHR_xcb_surface"], "KHR"),
    // platform surface extension: wayland
    Entrypoint::new(
        "create_wayland_surface",
        &[
            ("instance", "VkInstance"),
            ("create_info", "*const VkWaylandSurfaceCreateInfoKHR"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("surface_out", "*mut VkSurfaceKHR"),
        ],
    )
    .with_result()
    .for_extensions(&["VK_KHR_wayland_surface"], "KHR"),
    Entrypoint::new(
        "get_physical_device_wayland_presentation_support",
        &[
            ("physical_device", "VkPhysicalDevice"),
            ("queue_family_index", "u32"),
            ("display", "*mut c_void"),
        ],
    )
    .with_return("VkBool32")
    .for_extensions(&["VK_KHR_wayland_surface"], "KHR"),
    // platform surface extension: android
    Entrypoint::new(
        "create_android_surface",
        &[
            ("instance", "VkInstance"),
            ("create_info", "*const VkAndroidSurfaceCreateInfoKHR"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("surface_out", "*mut VkSurfaceKHR"),
        ],
    )
    .with_result()
    .for_extensions(&["VK_KHR_android_surface"], "KHR"),
    // platform surface extension: win32
    Entrypoint::new(
        "create_win32_surface",
        &[
            ("instance", "VkInstance"),
            ("create_info", "*const VkWin32SurfaceCreateInfoKHR"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("surface_out", "*mut VkSurfaceKHR"),
        ],
    )
    .with_result()
    .for_extensions(&["VK_KHR_win32_surface"], "KHR"),
    Entrypoint::new(
        "get_physical_device_win32_presentation_support",
        &[("physical_device", "VkPhysicalDevice"), ("queue_family_index", "u32")],
    )
    .with_return("VkBool32")
    .for_extensions(&["VK_KHR_win32_surface"], "KHR"),
    // platform surface extension: macos
    Entrypoint::new(
        "create_macos_surface",
        &[
            ("instance", "VkInstance"),
            ("create_info", "*const VkMacOSSurfaceCreateInfoMVK"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("surface_out", "*mut VkSurfaceKHR"),
        ],
    )
    .with_result()
    .for_extensions(&["VK_MVK_macos_surface"], "MVK")
    .canonical_name("vkCreateMacOSSurfaceMVK"),
    // display extension
    Entrypoint::new(
        "get_physical_device_display_properties",
        &[
            ("physical_device", "VkPhysicalDevice"),
            ("property_count_out", "*mut u32"),
            ("properties_out", "*mut VkDisplayPropertiesKHR"),
        ],
    )
    .with_result()
    .for_extensions(&["VK_KHR_display"], "KHR"),
    Entrypoint::new(
        "get_physical_device_display_plane_properties",
        &[
            ("physical_device", "VkPhysicalDevice"),
            ("property_count_out", "*mut u32"),
            ("properties_out", "*mut VkDisplayPlanePropertiesKHR"),
        ],
    )
    .with_result()
    .for_extensions(&["VK_KHR_display"], "KHR"),
    Entrypoint::new(
        "get_display_plane_supported_displays",
        &[
            ("physical_device", "VkPhysicalDevice"),
            ("plane_index", "u32"),
            ("display_count_out", "*mut u32"),
            ("displays_out", "*mut VkDisplayKHR"),
        ],
    )
    .with_result()
    .for_extensions(&["VK_KHR_display"], "KHR"),
    Entrypoint::new(
        "get_display_mode_properties",
        &[
            ("physical_device", "VkPhysicalDevice"),
            ("display", "VkDisplayKHR"),
            ("property_count_out", "*mut u32"),
            ("properties_out", "*mut VkDisplayModePropertiesKHR"),
        ],
    )
    .with_result()
    .for_extensions(&["VK_KHR_display"], "KHR"),
    Entrypoint::new(
        "create_display_mode",
        &[
            ("physical_device", "VkPhysicalDevice"),
            ("display", "VkDisplayKHR"),
            ("create_info", "*const VkDisplayModeCreateInfoKHR"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("mode_out", "*mut VkDisplayModeKHR"),
        ],
    )
    .with_result()
    .for_extensions(&["VK_KHR_display"], "KHR"),
    Entrypoint::new(
        "get_display_plane_capabilities",
        &[
            ("physcial_device", "VkPhysicalDevice"),
            ("mode", "VkDisplayModeKHR"),
            ("plane_index", "u32"),
            ("capabilities_out", "*mut VkDisplayPlaneCapabilitiesKHR"),
        ],
    )
    .with_result()
    .for_extensions(&["VK_KHR_display"], "KHR"),
    Entrypoint::new(
        "create_display_plane_surface",
        &[
            ("instance", "VkInstance"),
            ("create_info", "*const VkDisplaySurfaceCreateInfoKHR"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("surface_out", "*mut VkSurfaceKHR"),
        ],
    )
    .with_result()
    .for_extensions(&["VK_KHR_display", "VK_KHR_surface"], "KHR"),
    // 1.2 promoted
    Entrypoint::new(
        "create_render_pass2",
        &[
            ("device", "VkDevice"),
            ("create_info", "*const VkRenderPassCreateInfo2"),
            ("allocator", "*const VkAllocationCallbacks"),
            ("out", "*mut VkRenderPass"),
        ],
    )
    .with_result()
    .promoted_at(1, 2),
    Entrypoint::new(
        "cmd_begin_render_pass2",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("begin_info", "*const VkRenderPassBeginInfo"),
            ("begin_subpass_info", "*const VkSubpassBeginInfo"),
        ],
    )
    .promoted_at(1, 2),
    Entrypoint::new(
        "cmd_next_subpass2",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("begin_subpass_info", "*const VkSubpassBeginInfo"),
            ("end_subpass_info", "*const VkSubpassEndInfo"),
        ],
    )
    .promoted_at(1, 2),
    Entrypoint::new(
        "cmd_end_render_pass2",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("end_subpass_info", "*const VkSubpassEndInfo"),
        ],
    )
    .promoted_at(1, 2),
    // 1.3 promoted
    Entrypoint::new(
        "cmd_pipeline_barrier2",
        &[
            ("command_buffer", "VkCommandBuffer"),
            ("dependency_info", "*const VkDependencyInfo"),
        ],
    )
    .promoted_at(1, 3),
    Entrypoint::new(
        "queue_submit2",
        &[
            ("queue", "VkQueue"),
            ("submit_count", "u32"),
            ("submits", "*const VkSubmitInfo2"),
            ("fence", "VkFence"),
        ],
    )
    .with_result()
    .promoted_at(1, 3),
];

/* TODO: translate follows

#[cfg(feature = "Implements")]
pub trait OldResolverInterface {
    #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
    unsafe fn get_physical_device_surface_capabilities2_khr(
        &self,
        physicalDevice: VkPhysicalDevice,
        surface_info: *const VkPhysicalDeviceSurfaceInfo2KHR,
        surface_capabilities: *mut VkSurfaceCapabilities2KHR,
    ) -> VkResult;

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
    #[cfg(feature = "VK_KHR_device_group")]
    unsafe fn cmd_set_device_mask_khr(&self, commandBuffer: VkCommandBuffer, deviceMask: u32);
    #[cfg(feature = "VK_KHR_device_group")]
    unsafe fn cmd_dispatch_base_khr(
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
}
*/

fn main() {
    let mut function_pointer_table = String::with_capacity(8192);
    let mut fptbl_init = String::with_capacity(8192);
    function_pointer_table.push_str(
        r#"#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
struct FunctionPointerTable {
"#,
    );
    fptbl_init.push_str(
        r#"#[rustfmt::skip] #[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
impl FunctionPointerTable {
    const INIT: Self = Self {
"#,
    );
    let mut stubs = String::with_capacity(32 * 1024);
    let mut eps = String::with_capacity(16 * 1024);
    let mut emitted = false;
    for ep in CATALOG.iter() {
        if emitted {
            function_pointer_table.push_str(",\n");
            fptbl_init.push_str(",\n");
        }

        ep.gen_fptbl_entry(&mut function_pointer_table).unwrap();
        ep.gen_fptbl_init_entry(&mut fptbl_init).unwrap();
        ep.gen_stub(&mut stubs).unwrap();
        ep.gen_entrypoint(&mut eps).unwrap();
        emitted = true;
    }
    function_pointer_table.push_str(
        r#"
}
#[rustfmt::skip]
#[cfg(any(feature = "DynamicLoaded", feature = "CustomResolver"))]
static mut FPTBL: FunctionPointerTable = FunctionPointerTable::INIT;
"#,
    );
    fptbl_init
        .push_str("\n    };\n    #[inline(always)] #[rustfmt::skip] pub(crate) fn reset() { unsafe { FPTBL = Self::INIT; } }\n}\n");

    println!("use crate::vk::*;\nuse core::ffi::*;\n");
    println!("{eps}");
    println!("{function_pointer_table}");
    println!("{fptbl_init}");
    print!("{stubs}");
}
