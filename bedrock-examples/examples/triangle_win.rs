use std::rc::Rc;

use bedrock::{
    self as br, CommandBufferMut, CommandPoolMut, DescriptorPoolMut, DeviceMemoryMut, FenceMut, QueueMut, ShaderModule,
    VkHandle, VkHandleMut,
};
use br::{
    Device, Fence, GraphicsPipelineBuilder, ImageSubresourceSlice, Instance, MemoryBound, PhysicalDevice, RenderPass,
    Status, Swapchain,
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

fn read_spv_binary(path: impl AsRef<std::path::Path>) -> std::io::Result<Vec<u32>> {
    let b = std::fs::read(path)?;

    // 4byte境界にするためにとりあえずコピー
    // 本当はちゃんとあらかじめalignしたバッファ領域に直接読み込むような実装を作ったほうがいい
    let mut b2 = Vec::<u32>::with_capacity((b.len() + 3) >> 2);
    for x in b.chunks(4) {
        let b1 = match x {
            &[a, b, c, d] => [a, b, c, d],
            &[a, b, c] => [a, b, c, 0],
            &[a, b] => [a, b, 0, 0],
            &[a] => [a, 0, 0, 0],
            _ => unreachable!(),
        };
        b2.push(u32::from_ne_bytes(b1));
    }

    Ok(b2)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        SetProcessDPIAware();
    }

    let vk_version = br::instance_version()?;
    println!("vk instance version: {vk_version}");

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

    let instance = br::InstanceObject::new(&br::InstanceCreateInfo::new(
        &br::ApplicationInfo::new(
            c"BedrockExampleTriangle",
            br::Version::new(0, 0, 1, 0),
            c"None",
            br::Version::new(0, 0, 0, 1),
        )
        .api_version(br::Version::new(0, 1, 3, 0)),
        &[c"VK_LAYER_KHRONOS_validation".into()],
        &[
            c"VK_EXT_debug_utils".into(),
            c"VK_KHR_surface".into(),
            c"VK_KHR_win32_surface".into(),
        ],
    ))?;
    let adapter = instance
        .iter_physical_devices()?
        .next()
        .expect("No suitable adapter found");

    let _dbg = br::DebugUtilsMessengerObject::new(
        &instance,
        &br::DebugUtilsMessengerCreateInfo::new(
            br::vk::VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT
                | br::vk::VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT,
            br::vk::VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT
                | br::vk::VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT
                | br::vk::VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT,
            vk_debug,
        ),
    )?;

    let surface =
        unsafe { br::SurfaceObject::new(&adapter, &br::vk::VkWin32SurfaceCreateInfoKHR::new(cls.hInstance, w))? };

    let queue_families = adapter.queue_family_properties_alloc();
    let graphics_queue_family = queue_families
        .find_matching_index(br::QueueFlags::GRAPHICS)
        .expect("No graphics queue available");
    let device = br::DeviceObject::new(
        &adapter,
        &br::DeviceCreateInfo::new(
            &[br::DeviceQueueCreateInfo::new(graphics_queue_family, &[0.0])],
            &[],
            &[c"VK_KHR_swapchain".into()],
        )
        .with_next(
            &br::PhysicalDeviceFeatures2::new(Default::default())
                .with_next(&mut br::PhysicalDeviceSynchronization2Features::new(true)),
        ),
    )?;
    let mut queue = device.queue(graphics_queue_family, 0);

    if !adapter.surface_support(graphics_queue_family, &surface)? {
        panic!("Presentation is not supported on this queue");
    }

    let surface_caps = adapter.surface_capabilities(&surface)?;
    let surface_fmt = adapter.surface_formats_alloc(&surface)?;
    let surface_pm = adapter.surface_present_modes_alloc(&surface)?;
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
        .pre_transform(br::vk::VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR)
        .composite_alpha(br::vk::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR)
        .present_mode(present_mode)
        .create(&device)?,
    );
    let mut back_buffer_size = swapchain.size().clone();

    let render_pass = br::RenderPassObject::new(
        &device,
        &br::RenderPassCreateInfo2::new(
            &[br::AttachmentDescription2::new(fmt.format)
                .color_memory_op(br::LoadOp::Clear, br::StoreOp::Store)
                .layout_transition(br::ImageLayout::Undefined, br::ImageLayout::PresentSrc)],
            &[br::SubpassDescription2::new()
                .colors(&[br::AttachmentReference2::color(0, br::ImageLayout::ColorAttachmentOpt)])],
            &[
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
            ],
        ),
    )?;

    let descriptor_layout_ub1 = br::DescriptorSetLayoutObject::new(
        &device,
        &br::DescriptorSetLayoutCreateInfo::new(&[br::DescriptorType::UniformBuffer
            .make_binding(0, 1)
            .only_for_vertex()]),
    )?;
    let mut descriptor_pool = br::DescriptorPoolObject::new(
        &device,
        &br::DescriptorPoolCreateInfo::new(1, &[br::DescriptorType::UniformBuffer.make_size(1)]),
    )?;
    let descriptors = descriptor_pool.alloc(&[descriptor_layout_ub1.as_transparent_ref()])?;

    let vsh = br::ShaderModuleObject::new(
        &device,
        &br::ShaderModuleCreateInfo::new(&read_spv_binary("./examples/shaders/triangle.vspv")?),
    )?;
    let fsh = br::ShaderModuleObject::new(
        &device,
        &br::ShaderModuleCreateInfo::new(&read_spv_binary("./examples/shaders/triangle.fspv")?),
    )?;
    let pc = br::PipelineCacheObject::new(&device, &br::PipelineCacheCreateInfo::new(&[]))?;

    let scissors = [back_buffer_size.clone().into_rect(br::vk::VkOffset2D::ZERO)];
    let viewports = [scissors[0].make_viewport(0.0..1.0)];

    let pl = br::PipelineLayoutObject::new(
        &device,
        &br::PipelineLayoutCreateInfo::new(
            &[descriptor_layout_ub1.as_transparent_ref()],
            &[br::vk::VkPushConstantRange::for_type::<[f32; 2]>(
                br::vk::VK_SHADER_STAGE_VERTEX_BIT,
                0,
            )],
        ),
    )?;
    let vi_bindings = [br::vk::VkVertexInputBindingDescription::per_vertex_typed::<Vertex>(0)];
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
        let shader_stages = &[
            vsh.with_entry_point(c"main").on_stage(br::ShaderStage::Vertex),
            fsh.with_entry_point(c"main").on_stage(br::ShaderStage::Fragment),
        ];
        let vps = br::VertexProcessingStages::new(
            shader_stages,
            &vi_bindings,
            &vi_attributes,
            br::vk::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
        );
        let mut builder = br::NonDerivedGraphicsPipelineBuilder::new(&pl, render_pass.subpass(0), vps);
        builder
            .viewport_state(br::ViewportState::new(&viewports, &scissors))
            .multisample_state(Some(br::MultisampleState::new()))
            .color_blend_state(br::ColorBlendState::new(
                None,
                &[br::vk::VkPipelineColorBlendAttachmentState::PREMULTIPLIED],
                [0.0; 4],
            ));

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
        .map(|b| {
            br::FramebufferObject::new(
                &device,
                &br::FramebufferCreateInfo::new(
                    &render_pass,
                    &[b.as_transparent_ref()],
                    back_buffer_size.width,
                    back_buffer_size.height,
                ),
            )
        })
        .collect::<Result<Vec<_>, _>>()?;

    let memory_properties = adapter.memory_properties();
    let mut vbuf = br::BufferObject::new(
        &device,
        &br::BufferCreateInfo::new(
            core::mem::size_of::<Vertex>() * 3,
            br::BufferUsage::VERTEX_BUFFER.transfer_dest(),
        ),
    )?;
    let mut ubuf = br::BufferObject::new(
        &device,
        &br::BufferCreateInfo::new(
            core::mem::size_of::<f32>(),
            br::BufferUsage::UNIFORM_BUFFER.transfer_dest(),
        ),
    )?;
    let vbuf_requirements = vbuf.requirements();
    let ubuf_requirements = ubuf.requirements();
    let device_local_memory_index = memory_properties
        .find_device_local_index(vbuf_requirements.memoryTypeBits & ubuf_requirements.memoryTypeBits)
        .expect("No suitable memory for device local buffers");
    let vbuf_device_offset = 0u64;
    let ubuf_device_offset = (vbuf_requirements.size + (ubuf_requirements.alignment - 1)) / ubuf_requirements.alignment
        * ubuf_requirements.alignment;
    let device_memory = br::DeviceMemoryRequest::allocate(
        (ubuf_device_offset + ubuf_requirements.size) as _,
        device_local_memory_index,
    )
    .execute(&device)?;
    vbuf.bind(&device_memory, vbuf_device_offset as _)?;
    ubuf.bind(&device_memory, ubuf_device_offset as _)?;
    device.update_descriptor_sets(
        &[
            br::DescriptorPointer::new(descriptors[0].0, 0).write(br::DescriptorContents::UniformBuffer(vec![
                br::DescriptorBufferInfo::new(&ubuf, 0..core::mem::size_of::<f32>() as u64),
            ])),
        ],
        &[],
    );

    let host_buffer_size = ubuf_device_offset + ubuf_requirements.size;
    let mut host_buffer = br::BufferObject::new(
        &device,
        &br::BufferCreateInfo::new(host_buffer_size as _, br::BufferUsage::TRANSFER_SRC),
    )?;
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
            vbuf_device_offset as _,
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

    let mut command_pool = br::CommandPoolObject::new(&device, &br::CommandPoolCreateInfo::new(graphics_queue_family))?;
    let mut command_buffers = br::CommandBufferObject::alloc(
        &device,
        &br::CommandBufferAllocateInfo::new(
            &mut command_pool,
            framebuffers.len() as _,
            br::CommandBufferLevel::Primary,
        ),
    )?;
    for (cb, fb) in command_buffers.iter_mut().zip(framebuffers.iter()) {
        unsafe { cb.begin(&device)? }
            .begin_render_pass_2(
                &br::RenderPassBeginInfo::new(
                    &render_pass,
                    fb,
                    scissors[0].clone(),
                    &[br::ClearValue::color_f32([0.0, 0.0, 0.0, 1.0])],
                ),
                &br::vk::VkSubpassBeginInfo::new(br::vk::VK_SUBPASS_CONTENTS_INLINE),
            )
            .bind_graphics_pipeline(&pipeline)
            .bind_graphics_descriptor_sets(&pl, 0, &[descriptors[0]], &[])
            .push_constant(
                &pl,
                br::vk::VK_SHADER_STAGE_VERTEX_BIT,
                0,
                &[viewports[0].width, viewports[0].height],
            )
            .bind_vertex_buffers(0, &[vbuf.as_transparent_ref()], &[0])
            .draw(3, 1, 0, 0)
            .end_render_pass_2(&br::vk::VkSubpassEndInfo::new())
            .end()?;
    }

    let mut transfer_command_pool =
        br::CommandPoolObject::new(&device, &br::CommandPoolCreateInfo::new(graphics_queue_family))?;
    let mut transfer_command_buffers = br::CommandBufferObject::alloc_array(
        &device,
        &br::CommandBufferFixedCountAllocateInfo::<'_, 1>::new(
            &mut transfer_command_pool,
            br::CommandBufferLevel::Primary,
        ),
    )?;
    unsafe { transfer_command_buffers[0].begin(&device)? }
        .copy_buffer(
            &host_buffer,
            &ubuf,
            &[br::BufferCopy::copy_data::<f32>(ubuf_device_offset, 0)],
        )
        .pipeline_barrier_2(&br::DependencyInfo::new(
            &[br::MemoryBarrier2::new()
                .of_execution(br::PipelineStageFlags2::COPY, br::PipelineStageFlags2::VERTEX_SHADER)
                .of_memory(br::AccessFlags2::TRANSFER.write, br::AccessFlags2::UNIFORM_READ)],
            &[],
            &[],
        ))
        .end()?;

    let mut init_fence = br::FenceObject::new(&device, &br::FenceCreateInfo::new(0))?;
    let mut init_command_pool = br::CommandPoolObject::new(
        &device,
        &br::CommandPoolCreateInfo::new(graphics_queue_family).transient(),
    )?;
    let mut init_command_buffers = br::CommandBufferObject::alloc(
        &device,
        &br::CommandBufferAllocateInfo::new(&mut init_command_pool, 1, br::CommandBufferLevel::Primary),
    )?;
    unsafe { init_command_buffers[0].begin_once(&device)? }
        .copy_buffer(
            &host_buffer,
            &vbuf,
            &[br::BufferCopy::copy_data::<[Vertex; 3]>(vbuf_device_offset, 0)],
        )
        .copy_buffer(
            &host_buffer,
            &ubuf,
            &[br::BufferCopy::copy_data::<f32>(ubuf_device_offset, 0)],
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
        ))
        .end()?;
    queue.submit2(
        &[br::SubmitInfo2::new(
            &[],
            &[br::CommandBufferSubmitInfo::new(&init_command_buffers[0])],
            &[],
        )],
        Some(init_fence.as_transparent_ref_mut()),
    )?;
    init_fence.wait()?;

    let mut bb_ready = br::SemaphoreObject::new(&device, &br::SemaphoreCreateInfo::new())?;
    let data_ready = br::SemaphoreObject::new(&device, &br::SemaphoreCreateInfo::new())?;
    let present_ready = br::SemaphoreObject::new(&device, &br::SemaphoreCreateInfo::new())?;
    let mut last_render_fence = br::FenceObject::new(&device, &br::FenceCreateInfo::new(0))?;
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
            let surface_fmt = adapter.surface_formats_alloc(&surface)?;
            let surface_pm = adapter.surface_present_modes_alloc(&surface)?;
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
                .pre_transform(br::vk::VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR)
                .composite_alpha(br::vk::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR)
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
                .map(|b| {
                    br::FramebufferObject::new(
                        &device,
                        &br::FramebufferCreateInfo::new(
                            &render_pass,
                            &[b.as_transparent_ref()],
                            back_buffer_size.width,
                            back_buffer_size.height,
                        ),
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;

            let scissors = [back_buffer_size.clone().into_rect(br::vk::VkOffset2D::ZERO)];
            let viewports = [scissors[0].make_viewport(0.0..1.0)];

            pipeline = {
                let shader_stages = &[
                    vsh.with_entry_point(c"main").on_stage(br::ShaderStage::Vertex),
                    fsh.with_entry_point(c"main").on_stage(br::ShaderStage::Fragment),
                ];
                let vps = br::VertexProcessingStages::new(
                    shader_stages,
                    &vi_bindings,
                    &vi_attributes,
                    br::vk::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
                );
                let mut builder = br::NonDerivedGraphicsPipelineBuilder::new(&pl, render_pass.subpass(0), vps);
                builder
                    .viewport_state(br::ViewportState::new(&viewports, &scissors))
                    .multisample_state(Some(br::MultisampleState::new()))
                    .color_blend_state(br::ColorBlendState::new(
                        None,
                        &[br::vk::VkPipelineColorBlendAttachmentState::PREMULTIPLIED],
                        [0.0; 4],
                    ));

                builder.create(&device, Some(&pc))?
            };

            unsafe {
                command_pool.free(&command_buffers);
            }
            command_buffers = br::CommandBufferObject::alloc(
                &device,
                &br::CommandBufferAllocateInfo::new(
                    &mut command_pool,
                    framebuffers.len() as _,
                    br::CommandBufferLevel::Primary,
                ),
            )?;
            for (cb, fb) in command_buffers.iter_mut().zip(framebuffers.iter()) {
                unsafe { cb.begin(&device)? }
                    .begin_render_pass(
                        &render_pass,
                        fb,
                        scissors[0].clone(),
                        &[br::ClearValue::color_f32([0.0, 0.0, 0.0, 1.0])],
                        true,
                    )
                    .bind_graphics_pipeline(&pipeline)
                    .bind_graphics_descriptor_sets(&pl, 0, &[descriptors[0]], &[])
                    .push_constant(
                        &pl,
                        br::vk::VK_SHADER_STAGE_VERTEX_BIT,
                        0,
                        &[viewports[0].width, viewports[0].height],
                    )
                    .bind_vertex_buffers(0, &[vbuf.as_transparent_ref()], &[0])
                    .draw(3, 1, 0, 0)
                    .end_render_pass()
                    .end()?;
            }

            resize_next = false;
        }

        let bb_index =
            swapchain.acquire_next(None, br::CompletionHandlerMut::Queue(bb_ready.as_transparent_ref_mut()))?;

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
            Some(last_render_fence.as_transparent_ref_mut()),
        )?;
        match queue.present(br::PresentInfo::new(
            &[present_ready.as_transparent_ref()],
            &[swapchain.as_transparent_ref()],
            &[bb_index],
        )) {
            Err(e) if e == br::vk::VK_ERROR_OUT_OF_DATE_KHR => {
                resize_next = true;
            }
            x => {
                x?;
            }
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
