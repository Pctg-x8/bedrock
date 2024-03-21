use std::rc::Rc;

use bedrock as br;
use br::{
    CommandBuffer, CommandPool, DescriptorPool, Device, DeviceMemory, Fence, GraphicsPipelineBuilder,
    ImageSubresourceSlice, Instance, MemoryBound, PhysicalDevice, PipelineShaderStageProvider, Queue, Status,
    Swapchain, VulkanStructure,
};
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM},
        Graphics::Gdi::HBRUSH,
        System::LibraryLoader::GetModuleHandleA,
        UI::WindowsAndMessaging::{
            AdjustWindowRectEx, CreateWindowExA, DefWindowProcA, DispatchMessageA, LoadCursorA, LoadIconA,
            PeekMessageA, PostQuitMessage, RegisterClassExA, SetProcessDPIAware, TranslateMessage, CS_OWNDC,
            CW_USEDEFAULT, HICON, IDC_ARROW, IDI_APPLICATION, MSG, PM_REMOVE, WM_DESTROY, WM_QUIT, WNDCLASSEXA,
            WS_EX_APPWINDOW, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
        },
    },
};

#[repr(C)]
#[derive(Clone)]
pub struct Vertex {
    pub pos: [f32; 4],
    pub col: [f32; 4],
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        SetProcessDPIAware();
    }

    let instance_version = br::enumerate_instance_version()?;
    println!(
        "vk instance version: {instance_version} {}.{}.{}",
        br::vk::VK_MAJOR_VERSION(instance_version),
        br::vk::VK_MINOR_VERSION(instance_version),
        br::vk::VK_PATCH_VERSION(instance_version)
    );

    let cls = WNDCLASSEXA {
        cbSize: core::mem::size_of::<WNDCLASSEXA>() as _,
        cbClsExtra: 0,
        cbWndExtra: 0,
        style: CS_OWNDC,
        lpfnWndProc: Some(wndproc),
        hInstance: HINSTANCE(unsafe { GetModuleHandleA(PCSTR::null())?.0 }),
        hIcon: unsafe { LoadIconA(None, PCSTR(IDI_APPLICATION.0 as _))? },
        hCursor: unsafe { LoadCursorA(None, PCSTR(IDC_ARROW.0 as _))? },
        hbrBackground: HBRUSH(0),
        lpszMenuName: PCSTR::null(),
        lpszClassName: windows::core::s!("io.ct2.bedrock.example.triangle_win"),
        hIconSm: HICON(0),
    };
    let atom = unsafe { RegisterClassExA(&cls) };
    if atom == 0 {
        panic!("RegisterClassExA failed");
    }

    let ws = WS_OVERLAPPEDWINDOW | WS_VISIBLE;
    let wsx = WS_EX_APPWINDOW;
    let mut cr = RECT {
        top: 0,
        left: 0,
        right: 640,
        bottom: 480,
    };
    unsafe {
        AdjustWindowRectEx(&mut cr, ws, false, wsx)?;
    }
    let w = unsafe {
        CreateWindowExA(
            wsx,
            PCSTR(atom as usize as _),
            windows::core::s!("Bedrock Example: Triangle"),
            ws,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            cr.right - cr.left,
            cr.bottom - cr.top,
            None,
            None,
            cls.hInstance,
            None,
        )
    };

    let instance = {
        let mut builder = br::InstanceBuilder::new("BedrockExampleTriangle", (0, 1, 0), "None", (0, 0, 1));
        builder
            .set_api_version(1, 3, 0)
            .add_extensions(["VK_EXT_debug_utils", "VK_KHR_surface", "VK_KHR_win32_surface"])
            .add_layer("VK_LAYER_KHRONOS_validation");
        builder.create()?
    };
    let adapter = instance
        .iter_physical_devices()?
        .next()
        .expect("No suitable adapter found");

    let _dbg = br::DebugUtilsMessengerCreateInfo::new(vk_debug)
        .filter_severity(br::DebugUtilsMessageSeverityFlags::ERROR.and_warning())
        .filter_type(
            br::DebugUtilsMessageTypeFlags::GENERAL
                .and_performance()
                .and_validation(),
        )
        .create(&instance)?;

    let surface = (&adapter).new_surface_win32(cls.hInstance, w)?;

    let queue_families = adapter.queue_family_properties();
    let graphics_queue_family = queue_families
        .find_matching_index(br::QueueFlags::GRAPHICS)
        .expect("No graphics queue available");
    let device = {
        let qbinfo = br::DeviceQueueCreateInfo::new(graphics_queue_family).add(0.0);

        let mut builder = br::DeviceBuilder::new(&adapter);
        builder
            .add_queue(qbinfo)
            .add_extensions(["VK_KHR_swapchain"])
            .add_extra_features(br::vk::VkPhysicalDeviceSynchronization2Features {
                sType: br::vk::VkPhysicalDeviceSynchronization2Features::TYPE,
                pNext: core::ptr::null_mut(),
                synchronization2: br::vk::VK_TRUE,
            });

        builder.create()?
    };
    let mut queue = (&device).queue(graphics_queue_family, 0);

    if !adapter.surface_support(graphics_queue_family, &surface)? {
        panic!("Presentation is not supported on this queue");
    }

    let surface_caps = adapter.surface_capabilities(&surface)?;
    let surface_fmt = adapter.surface_formats(&surface)?;
    let surface_pm = adapter.surface_present_modes(&surface)?;
    let fmt = surface_fmt
        .iter()
        .find(|f| {
            br::FormatQuery(f.format)
                .eq_bit_width(32)
                .is_component_of(br::FormatComponents::RGBA)
                .has_element_of(br::ElementType::UNORM)
                .passed()
        })
        .or_else(|| {
            surface_fmt.iter().find(|f| {
                br::FormatQuery(f.format)
                    .eq_bit_width(32)
                    .is_component_of(br::FormatComponents::RGBA)
                    .has_element_of(br::ElementType::SRGB)
                    .passed()
            })
        })
        .expect("No expected formats supported");
    let present_mode = surface_pm[0];
    let mut swapchain = Rc::new(
        br::SwapchainBuilder::new(
            &surface,
            3.clamp(surface_caps.minImageCount, surface_caps.maxImageCount),
            fmt.clone(),
            surface_caps.currentExtent.clone(),
            br::ImageUsageFlags::COLOR_ATTACHMENT,
        )
        .pre_transform(br::SurfaceTransform::Identity)
        .composite_alpha(br::CompositeAlpha::Opaque)
        .present_mode(present_mode)
        .create(&device)?,
    );
    let mut back_buffer_size = swapchain.size().clone();

    let render_pass = {
        let attachments = [br::AttachmentDescription2::new(fmt.format)
            .color_memory_op(br::LoadOp::Clear, br::StoreOp::Store)
            .layout_transition(br::ImageLayout::Undefined, br::ImageLayout::PresentSrc)];
        let mainpass_color_outputs = [br::AttachmentReference2::color(0, br::ImageLayout::ColorAttachmentOpt)];
        let subpasses = [br::SubpassDescription2::new().colors(&mainpass_color_outputs)];
        let dependencies = [
            br::SubpassDependency2::new(br::SubpassIndex::External, br::SubpassIndex::Internal(0))
                .of_execution(
                    br::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                    br::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                )
                .of_memory(0, br::AccessFlags::COLOR_ATTACHMENT.write)
                .by_region(),
            br::SubpassDependency2::new(br::SubpassIndex::Internal(0), br::SubpassIndex::External)
                .of_execution(
                    br::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                    br::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                )
                .of_memory(br::AccessFlags::COLOR_ATTACHMENT.write, 0)
                .by_region(),
        ];

        br::RenderPassBuilder2::new(&attachments, &subpasses, &dependencies).create(&device)?
    };

    let descriptor_layout_ub1 = br::DescriptorSetLayoutBuilder::new()
        .bind(br::DescriptorType::UniformBuffer.make_binding(1).only_for_vertex())
        .create(&device)?;
    let mut descriptor_pool = br::DescriptorPoolBuilder::new(1)
        .reserve(br::DescriptorType::UniformBuffer.with_count(1))
        .create(&device)?;
    let descriptors = descriptor_pool.alloc(&[&descriptor_layout_ub1])?;

    let vsh = (&device).new_shader_module(&std::fs::read("./examples/shaders/triangle.vspv")?)?;
    let fsh = (&device).new_shader_module(&std::fs::read("./examples/shaders/triangle.fspv")?)?;
    let pc = (&device).new_pipeline_cache(&[])?;

    let scissors = [back_buffer_size.clone().into_rect(br::vk::VkOffset2D::ZERO)];
    let viewports = [scissors[0].make_viewport(0.0..1.0)];

    let pl = br::PipelineLayoutBuilder::new(vec![&descriptor_layout_ub1], vec![(br::ShaderStage::VERTEX, 0..4 * 2)])
        .create(&device)?;
    let vi_bindings = [br::VertexInputBindingDescription::per_vertex_typed::<Vertex>(0)];
    let vi_attributes = [
        br::vk::VkVertexInputAttributeDescription {
            location: 0,
            binding: 0,
            format: br::vk::VK_FORMAT_R32G32B32A32_SFLOAT,
            offset: 0,
        },
        br::vk::VkVertexInputAttributeDescription {
            location: 1,
            binding: 0,
            format: br::vk::VK_FORMAT_R32G32B32A32_SFLOAT,
            offset: 4 * 4,
        },
    ];
    let mut pipeline = {
        let shader_stages =
            br::VertexShaderStage::new(br::PipelineShader2::new(&vsh, std::ffi::CString::new("main").unwrap()))
                .with_fragment_shader_stage(br::PipelineShader2::new(&fsh, std::ffi::CString::new("main").unwrap()));
        let vps = br::VertexProcessingStages::new(
            shader_stages,
            &vi_bindings,
            &vi_attributes,
            br::vk::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
        );
        let mut builder = br::NonDerivedGraphicsPipelineBuilder::new(&pl, (&render_pass, 0), vps);
        builder
            .viewport_scissors(
                br::DynamicArrayState::Static(&viewports),
                br::DynamicArrayState::Static(&scissors),
            )
            .multisample_state(Some(br::MultisampleState::new()))
            .add_attachment_blend(br::AttachmentColorBlendState::premultiplied());

        builder.create(&device, Some(&pc))?
    };

    let mut back_buffer_views = swapchain
        .get_images()?
        .into_iter()
        .map(|b| {
            b.clone_parent()
                .subresource_range(br::AspectMask::COLOR, 0..1, 0..1)
                .view_builder()
                .create()
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut framebuffers = back_buffer_views
        .iter()
        .map(|b| br::FramebufferBuilder::new_with_attachment(&render_pass, b).create())
        .collect::<Result<Vec<_>, _>>()?;

    let memory_properties = adapter.memory_properties();
    let mut vbuf = br::BufferDesc::new(
        core::mem::size_of::<Vertex>() * 3,
        br::BufferUsage::VERTEX_BUFFER.transfer_dest(),
    )
    .create(&device)?;
    let mut ubuf = br::BufferDesc::new(
        core::mem::size_of::<f32>(),
        br::BufferUsage::UNIFORM_BUFFER.transfer_dest(),
    )
    .create(&device)?;
    let vbuf_requirements = vbuf.requirements();
    let ubuf_requirements = ubuf.requirements();
    let device_local_memory_index = memory_properties
        .find_device_local_index(vbuf_requirements.memoryTypeBits & ubuf_requirements.memoryTypeBits)
        .expect("No suitable memory for device local buffers");
    let vbuf_device_offset = 0;
    let ubuf_device_offset = (vbuf_requirements.size + (ubuf_requirements.alignment - 1)) / ubuf_requirements.alignment
        * ubuf_requirements.alignment;
    let device_memory = br::DeviceMemoryRequest::allocate(
        (ubuf_device_offset + ubuf_requirements.size) as _,
        device_local_memory_index,
    )
    .execute(&device)?;
    vbuf.bind(&device_memory, vbuf_device_offset)?;
    ubuf.bind(&device_memory, ubuf_device_offset as _)?;
    device.update_descriptor_sets(
        &[
            br::DescriptorPointer::new(descriptors[0].0, 0).write(br::DescriptorContents::UniformBuffer(vec![
                br::DescriptorBufferRef::new(&ubuf, 0..core::mem::size_of::<f32>() as u64),
            ])),
        ],
        &[],
    );

    let host_buffer_size = ubuf_device_offset + ubuf_requirements.size;
    let mut host_buffer = br::BufferDesc::new(host_buffer_size as _, br::BufferUsage::TRANSFER_SRC).create(&device)?;
    let host_buffer_requirements = host_buffer.requirements();
    let host_memory_index = memory_properties
        .find_host_visible_index(host_buffer_requirements.memoryTypeBits)
        .expect("No suitable memory for init buffer");
    let mut host_memory =
        br::DeviceMemoryRequest::allocate(host_buffer_requirements.size as _, host_memory_index).execute(&device)?;
    host_buffer.bind(&host_memory, 0)?;
    let p = host_memory.map(0..host_buffer_requirements.size as _)?;
    unsafe {
        p.clone_from_slice_at(
            vbuf_device_offset,
            &[
                Vertex {
                    pos: [
                        100.0 * (-90.0f32).to_radians().cos(),
                        100.0 * (-90.0f32).to_radians().sin(),
                        0.0,
                        1.0,
                    ],
                    col: [1.0, 1.0, 1.0, 1.0],
                },
                Vertex {
                    pos: [
                        100.0 * (-210.0f32).to_radians().cos(),
                        100.0 * (-210.0f32).to_radians().sin(),
                        0.0,
                        1.0,
                    ],
                    col: [1.0, 0.75, 0.0, 1.0],
                },
                Vertex {
                    pos: [
                        100.0 * 30.0f32.to_radians().cos(),
                        100.0 * 30.0f32.to_radians().sin(),
                        0.0,
                        1.0,
                    ],
                    col: [0.0, 0.75, 1.0, 1.0],
                },
            ],
        );
        *p.get_mut(ubuf_device_offset as _) = 0.0f32;

        host_memory.unmap();
    }

    let mut command_pool = br::CommandPoolBuilder::new(graphics_queue_family).create(&device)?;
    let mut command_buffers = command_pool.alloc(framebuffers.len() as _, true)?;
    for (cb, fb) in command_buffers.iter_mut().zip(framebuffers.iter()) {
        let mut rec = unsafe { cb.begin()? };
        rec.begin_render_pass_2(
            &br::RenderPassBeginInfo::new(
                &render_pass,
                fb,
                scissors[0].clone(),
                &[br::ClearValue::color_f32([0.0, 0.0, 0.0, 1.0])],
            ),
            &br::SubpassBeginInfo::new(br::vk::VK_SUBPASS_CONTENTS_INLINE),
        )
        .bind_graphics_pipeline_pair(&pipeline, &pl)
        .bind_graphics_descriptor_sets(0, &[descriptors[0].0], &[])
        .push_graphics_constant(br::ShaderStage::VERTEX, 0, &[viewports[0].width, viewports[0].height])
        .bind_vertex_buffers(0, &[(&vbuf, 0)])
        .draw(3, 1, 0, 0)
        .end_render_pass_2(&br::SubpassEndInfo::new());
        rec.end()?;
    }

    let mut transfer_command_pool = br::CommandPoolBuilder::new(graphics_queue_family).create(&device)?;
    let mut transfer_command_buffers = transfer_command_pool.alloc(1, true)?;
    let mut rec = unsafe { transfer_command_buffers[0].begin()? };
    rec.copy_buffer(
        &host_buffer,
        &ubuf,
        &[br::vk::VkBufferCopy {
            srcOffset: ubuf_device_offset as _,
            dstOffset: 0,
            size: core::mem::size_of::<f32>() as _,
        }],
    )
    .pipeline_barrier_2(&br::DependencyInfo::new(
        &[br::MemoryBarrier2::new()
            .of_execution(br::PipelineStageFlags2::COPY, br::PipelineStageFlags2::VERTEX_SHADER)
            .of_memory(br::AccessFlags2::TRANSFER.write, br::AccessFlags2::UNIFORM_READ)],
        &[],
        &[],
    ));
    rec.end()?;

    let mut init_fence = br::FenceBuilder::new().create(&device)?;
    let mut init_command_pool = br::CommandPoolBuilder::new(graphics_queue_family)
        .transient()
        .create(&device)?;
    let mut init_command_buffers = init_command_pool.alloc(1, true)?;
    let mut init_rec = unsafe { init_command_buffers[0].begin_once()? };
    init_rec
        .copy_buffer(
            &host_buffer,
            &vbuf,
            &[br::vk::VkBufferCopy {
                srcOffset: vbuf_device_offset as _,
                dstOffset: 0,
                size: core::mem::size_of::<Vertex>() as u64 * 3,
            }],
        )
        .copy_buffer(
            &host_buffer,
            &ubuf,
            &[br::vk::VkBufferCopy {
                srcOffset: ubuf_device_offset as _,
                dstOffset: 0,
                size: core::mem::size_of::<f32>() as u64,
            }],
        )
        .pipeline_barrier_2(&br::DependencyInfo::new(
            &[br::MemoryBarrier2::new()
                .of_execution(
                    br::PipelineStageFlags2::COPY,
                    br::PipelineStageFlags2::VERTEX_ATTRIBUTE_INPUT | br::PipelineStageFlags2::VERTEX_SHADER,
                )
                .of_memory(
                    br::AccessFlags2::TRANSFER.write,
                    br::AccessFlags2::VERTEX_ATTRIBUTE_READ | br::AccessFlags2::UNIFORM_READ,
                )],
            &[],
            &[],
        ));
    init_rec.end()?;
    queue.submit2(
        &[br::SubmitInfo2::new(
            &[],
            &[br::CommandBufferSubmitInfo::new(&init_command_buffers[0])],
            &[],
        )],
        Some(&mut init_fence),
    )?;
    init_fence.wait()?;

    let bb_ready = br::SemaphoreBuilder::new().create(&device)?;
    let data_ready = br::SemaphoreBuilder::new().create(&device)?;
    let present_ready = br::SemaphoreBuilder::new().create(&device)?;
    let mut last_render_fence = br::FenceBuilder::new().create(&device)?;
    let mut last_render_occured = false;
    let mut resize_next = false;

    let mut rot = 0.0f32;
    let mut msg = core::mem::MaybeUninit::<MSG>::uninit();
    let mut t = std::time::Instant::now();
    'lp: loop {
        while unsafe { PeekMessageA(msg.as_mut_ptr(), None, 0, 0, PM_REMOVE).0 } != 0 {
            unsafe {
                if msg.assume_init_ref().message == WM_QUIT {
                    break 'lp;
                }

                TranslateMessage(msg.as_ptr());
                DispatchMessageA(msg.as_ptr());
            }
        }

        if last_render_occured && !last_render_fence.status()? {
            continue;
        }
        last_render_fence.reset()?;

        if resize_next {
            drop(framebuffers);
            drop(back_buffer_views);
            drop(swapchain);

            let surface_caps = adapter.surface_capabilities(&surface)?;
            let surface_fmt = adapter.surface_formats(&surface)?;
            let surface_pm = adapter.surface_present_modes(&surface)?;
            let fmt = surface_fmt
                .iter()
                .find(|f| {
                    br::FormatQuery(f.format)
                        .eq_bit_width(32)
                        .is_component_of(br::FormatComponents::RGBA)
                        .has_element_of(br::ElementType::UNORM)
                        .passed()
                })
                .or_else(|| {
                    surface_fmt.iter().find(|f| {
                        br::FormatQuery(f.format)
                            .eq_bit_width(32)
                            .is_component_of(br::FormatComponents::RGBA)
                            .has_element_of(br::ElementType::SRGB)
                            .passed()
                    })
                })
                .expect("No expected formats supported");
            let present_mode = surface_pm[0];
            swapchain = Rc::new(
                br::SwapchainBuilder::new(
                    &surface,
                    3.clamp(surface_caps.minImageCount, surface_caps.maxImageCount),
                    fmt.clone(),
                    surface_caps.currentExtent.clone(),
                    br::ImageUsageFlags::COLOR_ATTACHMENT,
                )
                .pre_transform(br::SurfaceTransform::Identity)
                .composite_alpha(br::CompositeAlpha::Opaque)
                .present_mode(present_mode)
                .create(&device)?,
            );
            back_buffer_size = swapchain.size().clone();

            back_buffer_views = swapchain
                .get_images()?
                .into_iter()
                .map(|b| {
                    b.clone_parent()
                        .subresource_range(br::AspectMask::COLOR, 0..1, 0..1)
                        .view_builder()
                        .create()
                })
                .collect::<Result<Vec<_>, _>>()?;
            framebuffers = back_buffer_views
                .iter()
                .map(|b| br::FramebufferBuilder::new_with_attachment(&render_pass, b).create())
                .collect::<Result<Vec<_>, _>>()?;

            let scissors = [back_buffer_size.clone().into_rect(br::vk::VkOffset2D::ZERO)];
            let viewports = [scissors[0].make_viewport(0.0..1.0)];

            pipeline = {
                let shader_stages =
                    br::VertexShaderStage::new(br::PipelineShader2::new(&vsh, std::ffi::CString::new("main").unwrap()))
                        .with_fragment_shader_stage(br::PipelineShader2::new(
                            &fsh,
                            std::ffi::CString::new("main").unwrap(),
                        ));
                let vps = br::VertexProcessingStages::new(
                    shader_stages,
                    &vi_bindings,
                    &vi_attributes,
                    br::vk::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
                );
                let mut builder = br::NonDerivedGraphicsPipelineBuilder::new(&pl, (&render_pass, 0), vps);
                builder
                    .viewport_scissors(
                        br::DynamicArrayState::Static(&viewports),
                        br::DynamicArrayState::Static(&scissors),
                    )
                    .multisample_state(Some(br::MultisampleState::new()))
                    .add_attachment_blend(br::AttachmentColorBlendState::premultiplied());

                builder.create(&device, Some(&pc))?
            };

            unsafe {
                command_pool.free(&command_buffers);
            }
            command_buffers = command_pool.alloc(framebuffers.len() as _, true)?;
            for (cb, fb) in command_buffers.iter_mut().zip(framebuffers.iter()) {
                let mut rec = unsafe { cb.begin()? };
                rec.begin_render_pass(
                    &render_pass,
                    fb,
                    scissors[0].clone(),
                    &[br::ClearValue::color_f32([0.0, 0.0, 0.0, 1.0])],
                    true,
                )
                .bind_graphics_pipeline_pair(&pipeline, &pl)
                .bind_graphics_descriptor_sets(0, &[descriptors[0].0], &[])
                .push_graphics_constant(br::ShaderStage::VERTEX, 0, &[viewports[0].width, viewports[0].height])
                .bind_vertex_buffers(0, &[(&vbuf, 0)])
                .draw(3, 1, 0, 0)
                .end_render_pass();
                rec.end()?;
            }

            resize_next = false;
        }

        let bb_index = swapchain.acquire_next(
            None,
            br::CompletionHandler::<br::FenceObject<br::DeviceObject<br::InstanceObject>>, _>::Queue(&bb_ready),
        )?;

        let dt = t.elapsed().as_secs_f32();
        rot += dt * 120.0;
        t = std::time::Instant::now();

        let p = host_memory.map(0..host_buffer_size as _)?;
        unsafe {
            *p.get_mut(ubuf_device_offset as _) = rot.to_radians();
            host_memory.unmap();
        }

        let transfer_commands = [br::CommandBufferSubmitInfo::new(&transfer_command_buffers[0])];
        let transfer_done_semaphores = [br::SemaphoreSubmitInfo::new(&data_ready).on_vertex_shader()];
        let render_commands = [br::CommandBufferSubmitInfo::new(&command_buffers[bb_index as usize])];
        let render_wait_semaphores = [
            br::SemaphoreSubmitInfo::new(&bb_ready).on_color_attachment_output(),
            br::SemaphoreSubmitInfo::new(&data_ready).on_vertex_shader(),
        ];
        let render_done_semaphores = [br::SemaphoreSubmitInfo::new(&present_ready).on_color_attachment_output()];
        queue.submit2(
            &[
                br::SubmitInfo2::new(&[], &transfer_commands, &transfer_done_semaphores),
                br::SubmitInfo2::new(&render_wait_semaphores, &render_commands, &render_done_semaphores),
            ],
            Some(&mut last_render_fence),
        )?;
        match swapchain.queue_present(&mut queue, bb_index, &[&present_ready]) {
            Err(e) if e == br::vk::VK_ERROR_OUT_OF_DATE_KHR => {
                resize_next = true;
            }
            x => x?,
        };
        last_render_occured = true;
    }

    unsafe {
        device.wait()?;
    }
    // drop buffers before memory
    drop(host_buffer);
    drop(vbuf);
    drop(ubuf);

    Ok(())
}

extern "system" fn wndproc(hwnd: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    if msg == WM_DESTROY {
        unsafe {
            PostQuitMessage(0);
        }
        return LRESULT(0);
    }

    unsafe { DefWindowProcA(hwnd, msg, wp, lp) }
}

extern "system" fn vk_debug(
    _message_severity: br::vk::VkDebugUtilsMessageSeverityFlagBitsEXT,
    _message_types: br::vk::VkDebugUtilsMessageTypeFlagsEXT,
    callback_data: *const br::vk::VkDebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut std::ffi::c_void,
) -> br::vk::VkBool32 {
    eprintln!("[vk_debug] {}", unsafe {
        std::ffi::CStr::from_ptr(callback_data.as_ref().unwrap().pMessage)
            .to_str()
            .unwrap()
    });

    br::vk::VK_FALSE
}