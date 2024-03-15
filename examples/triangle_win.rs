#![cfg(not(test))]

use std::rc::Rc;

use bedrock as br;
use br::{
    CommandBuffer, CommandPool, DescriptorPool, Device, DeviceMemory, Fence, GraphicsPipelineBuilder,
    ImageSubresourceSlice, Instance, MemoryBound, PhysicalDevice, PipelineShaderStageProvider, Queue, Status,
    SubmissionBatch, Swapchain,
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
            .add_extensions(["VK_EXT_debug_utils", "VK_KHR_surface", "VK_KHR_win32_surface"])
            .add_layer("VK_LAYER_KHRONOS_validation");
        builder.create()?
    };
    let adapter = instance
        .iter_physical_devices()?
        .next()
        .expect("No suitable adapter found");

    let surface = (&adapter).new_surface_win32(cls.hInstance, w)?;

    let queue_families = adapter.queue_family_properties();
    let graphics_queue_family = queue_families
        .find_matching_index(br::QueueFlags::GRAPHICS)
        .expect("No graphics queue available");
    let device = {
        let qbinfo = br::DeviceQueueCreateInfo::new(graphics_queue_family).add(0.0);

        let mut builder = br::DeviceBuilder::new(&adapter);
        builder.add_queue(qbinfo).add_extensions(["VK_KHR_swapchain"]);

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
        let attachment =
            br::AttachmentDescription::new(fmt.format, br::ImageLayout::PresentSrc, br::ImageLayout::PresentSrc)
                .color_memory_op(br::LoadOp::Clear, br::StoreOp::Store);
        let main_subpass = br::SubpassDescription::new().add_color_output(0, br::ImageLayout::ColorAttachmentOpt, None);
        let enter_main_deps = br::vk::VkSubpassDependency {
            srcSubpass: br::vk::VK_SUBPASS_EXTERNAL,
            dstSubpass: 0,
            srcStageMask: br::PipelineStageFlags::ALL_COMMANDS.0,
            dstStageMask: br::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT.0,
            srcAccessMask: 0,
            dstAccessMask: br::AccessFlags::COLOR_ATTACHMENT.write,
            dependencyFlags: br::vk::VK_DEPENDENCY_BY_REGION_BIT,
        };
        let leave_main_deps = br::vk::VkSubpassDependency {
            srcSubpass: 0,
            dstSubpass: br::vk::VK_SUBPASS_EXTERNAL,
            srcStageMask: br::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT.0,
            dstStageMask: br::PipelineStageFlags::ALL_COMMANDS.0,
            srcAccessMask: br::AccessFlags::COLOR_ATTACHMENT.write,
            dstAccessMask: 0,
            dependencyFlags: br::vk::VK_DEPENDENCY_BY_REGION_BIT,
        };

        br::RenderPassBuilder::new()
            .add_attachment(attachment)
            .add_subpass(main_subpass)
            .add_dependencies([enter_main_deps, leave_main_deps])
            .create(&device)?
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

    let mut transfer_command_pool = br::CommandPoolBuilder::new(graphics_queue_family).create(&device)?;
    let mut transfer_command_buffers = transfer_command_pool.alloc(1, true)?;
    let mut rec = unsafe { transfer_command_buffers[0].begin()? };
    let ubuf_transfer_barrier = br::BufferMemoryBarrier::new(
        &ubuf,
        0..core::mem::size_of::<f32>() as u64,
        br::AccessFlags::UNIFORM_READ,
        br::AccessFlags::TRANSFER.write,
    );
    let host_transfer_barrier = br::BufferMemoryBarrier::new(
        &host_buffer,
        ubuf_device_offset..ubuf_device_offset + core::mem::size_of::<f32>() as u64,
        br::AccessFlags::HOST.write,
        br::AccessFlags::TRANSFER.read,
    );
    rec.pipeline_barrier(
        br::PipelineStageFlags::VERTEX_SHADER.host(),
        br::PipelineStageFlags::TRANSFER,
        false,
        &[],
        &[ubuf_transfer_barrier.clone(), host_transfer_barrier.clone()],
        &[],
    )
    .copy_buffer(
        &host_buffer,
        &ubuf,
        &[br::vk::VkBufferCopy {
            srcOffset: ubuf_device_offset as _,
            dstOffset: 0,
            size: core::mem::size_of::<f32>() as _,
        }],
    )
    .pipeline_barrier(
        br::PipelineStageFlags::TRANSFER,
        br::PipelineStageFlags::VERTEX_SHADER.host(),
        false,
        &[],
        &[ubuf_transfer_barrier.flip(), host_transfer_barrier.flip()],
        &[],
    );
    rec.end()?;

    let mut init_fence = br::FenceBuilder::new().create(&device)?;
    let mut init_command_pool = br::CommandPoolBuilder::new(graphics_queue_family)
        .transient()
        .create(&device)?;
    let mut init_command_buffers = init_command_pool.alloc(1, true)?;
    let mut init_rec = unsafe { init_command_buffers[0].begin_once()? };
    let vbuf_entire_barrier = br::BufferMemoryBarrier::new(
        &vbuf,
        0..core::mem::size_of::<Vertex>() as u64 * 3,
        0,
        br::AccessFlags::TRANSFER.write,
    );
    let ubuf_entire_barrier = br::BufferMemoryBarrier::new(
        &ubuf,
        0..core::mem::size_of::<f32>() as u64,
        0,
        br::AccessFlags::TRANSFER.write,
    );
    let back_buffer_init_transfers = swapchain
        .get_images()?
        .iter()
        .map(|b| {
            b.subresource_range(br::AspectMask::COLOR, 0..1, 0..1)
                .memory_barrier(br::ImageLayout::Undefined, br::ImageLayout::PresentSrc)
        })
        .collect::<Vec<_>>();
    init_rec
        .pipeline_barrier(
            br::PipelineStageFlags::BOTTOM_OF_PIPE.host(),
            br::PipelineStageFlags::TRANSFER,
            false,
            &[],
            &[
                vbuf_entire_barrier.clone(),
                ubuf_entire_barrier.clone(),
                br::BufferMemoryBarrier::new(
                    &host_buffer,
                    0..host_buffer_size,
                    br::AccessFlags::HOST.write,
                    br::AccessFlags::TRANSFER.read,
                ),
            ],
            &[],
        )
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
        .pipeline_barrier(
            br::PipelineStageFlags::TRANSFER,
            br::PipelineStageFlags::VERTEX_INPUT
                .vertex_shader()
                .color_attachment_output()
                .host(),
            false,
            &[],
            &[
                vbuf_entire_barrier
                    .flip()
                    .dest_access_mask(br::AccessFlags::VERTEX_ATTRIBUTE_READ),
                ubuf_entire_barrier
                    .flip()
                    .dest_access_mask(br::AccessFlags::UNIFORM_READ),
                br::BufferMemoryBarrier::new(
                    &host_buffer,
                    ubuf_device_offset..ubuf_device_offset + core::mem::size_of::<f32>() as u64,
                    br::AccessFlags::TRANSFER.read,
                    br::AccessFlags::HOST.write,
                ),
            ],
            &back_buffer_init_transfers,
        );
    init_rec.end()?;
    queue.submit(
        &[br::EmptySubmissionBatch.with_command_buffers(&init_command_buffers)],
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

            let mut init_fence = br::FenceBuilder::new().create(&device)?;
            let mut init_command_pool = br::CommandPoolBuilder::new(graphics_queue_family)
                .transient()
                .create(&device)?;
            let mut init_command_buffers = init_command_pool.alloc(1, true)?;
            let mut init_rec = unsafe { init_command_buffers[0].begin_once()? };
            let back_buffer_init_transfers = swapchain
                .get_images()?
                .iter()
                .map(|b| {
                    b.subresource_range(br::AspectMask::COLOR, 0..1, 0..1)
                        .memory_barrier(br::ImageLayout::Undefined, br::ImageLayout::PresentSrc)
                })
                .collect::<Vec<_>>();
            init_rec.pipeline_barrier(
                br::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                br::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                false,
                &[],
                &[],
                &back_buffer_init_transfers,
            );
            init_rec.end()?;
            queue.submit(
                &[br::EmptySubmissionBatch.with_command_buffers(&init_command_buffers)],
                Some(&mut init_fence),
            )?;
            init_fence.wait()?;

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

        let transfer_commands = &[transfer_command_buffers[0]];
        let transfer_ready_semaphores = &[&data_ready];
        let render_commands = &[command_buffers[bb_index as usize]];
        let render_wait_semaphores = &[
            (&bb_ready, br::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT),
            (&data_ready, br::PipelineStageFlags::VERTEX_SHADER),
        ];
        let render_ready_semaphores = &[&present_ready];
        let transfer_submission = br::EmptySubmissionBatch
            .with_command_buffers(transfer_commands)
            .with_signal_semaphores(transfer_ready_semaphores);
        let render_submission = br::EmptySubmissionBatch
            .with_command_buffers(render_commands)
            .with_wait_semaphores(render_wait_semaphores)
            .with_signal_semaphores(render_ready_semaphores);
        queue.submit(
            &[
                Box::new(transfer_submission) as Box<dyn br::SubmissionBatch>,
                Box::new(render_submission),
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
