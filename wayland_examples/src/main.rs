use bedrock::{
    self as br, CommandBufferMut, DescriptorPoolMut, Device, DeviceMemoryMut, Fence, FenceMut, GraphicsPipelineBuilder,
    ImageSubresourceSlice, Instance, MemoryBound, PhysicalDevice, QueueMut, RenderPass, ShaderModule, Swapchain,
    VkHandle, VkHandleMut,
};
use core::ffi::*;
use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

struct WaylandInterfaces {
    pub compositor: Option<&'static mut WlCompositor>,
    pub xdg_wm_base: Option<&'static mut XDGWmBase>,
}

#[repr(C)]
#[derive(Clone)]
pub struct Vertex {
    pub pos: [f32; 4],
    pub col: [f32; 4],
}

#[repr(C)]
pub struct ObjectParameters {
    pub rot: f32,
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

fn main() {
    let mut display = WlDisplayConnection::new(None);

    let registry = display.get_registry();
    struct RegistryListenerInstance {
        interfaces: WaylandInterfaces,
    }
    impl RegistryListener for RegistryListenerInstance {
        fn global(&mut self, obj: &mut WlRegistry, name: u32, interface: &core::ffi::CStr, version: u32) {
            let interface_name = interface.to_str().unwrap();

            println!("[wayland global registry] {interface_name} version: {version}, name: {name}");

            if interface_name == "wl_compositor" {
                if self.interfaces.compositor.is_some() {
                    panic!("one or more wl_compositor found");
                }

                let id = obj.bind(name, unsafe { &wl_compositor_interface }, version);
                assert!(!id.is_null(), "failed to bind wl_compositor");

                self.interfaces.compositor = Some(unsafe { &mut *(id as *mut WlCompositor) });
            }

            if interface_name == "xdg_wm_base" {
                if self.interfaces.xdg_wm_base.is_some() {
                    panic!("one or more xdg_wm_base found");
                }

                let id = obj.bind(name, &XDG_WM_BASE_INTERFACE, version);
                assert!(!id.is_null(), "failed to bind xdg_wm_base");

                self.interfaces.xdg_wm_base = Some(unsafe { &mut *(id as *mut XDGWmBase) });
            }
        }

        fn global_remove(&mut self, _obj: &mut WlRegistry, name: u32) {
            println!("[wayland global registry remove] name: {name}");
        }
    }
    let mut registry_listener = RegistryListenerInstance {
        interfaces: WaylandInterfaces {
            compositor: None,
            xdg_wm_base: None,
        },
    };
    registry.add_listener(&mut registry_listener);
    display.roundtrip();

    let compositor = registry_listener
        .interfaces
        .compositor
        .expect("no wl_compositor found in a roundtrip");
    let xdg_wm_base = registry_listener
        .interfaces
        .xdg_wm_base
        .expect("no xdg_wm_base found in a roundtrip");
    let mut surface = compositor.create_surface();
    let mut xdg_surface = xdg_wm_base.get_xdg_surface(&mut surface);
    struct XDGSurfaceListenerInstance;
    impl XDGSurfaceListener for XDGSurfaceListenerInstance {
        fn configure(&mut self, obj: &mut XDGSurface, serial: c_uint) {
            println!("surface configure: {serial}");
            obj.ack_configure(serial);
        }
    }
    xdg_surface.add_listener(&mut XDGSurfaceListenerInstance);
    let mut xdg_toplevel = xdg_surface.get_toplevel();
    struct XDGToplevelListenerInstance;
    impl XDGToplevelListener for XDGToplevelListenerInstance {
        fn configure(&mut self, _obj: &mut XDGToplevel, width: c_int, height: c_int, states: *mut wl_array) {
            let states = unsafe { (*states).as_slice_unchecked::<c_int>() };

            println!("toplevel configure: {width} {height} {states:?}");
        }

        fn close(&mut self, _obj: &mut XDGToplevel) {
            println!("toplevel close");
        }

        fn configure_bounds(&mut self, _obj: &mut XDGToplevel, width: c_int, height: c_int) {
            println!("toplevel configure_bounds: {width} {height}");
        }

        fn wm_capabilities(&mut self, _obj: &mut XDGToplevel, capabilities: *mut wl_array) {
            let caps = unsafe { (*capabilities).as_slice_unchecked::<c_int>() };

            println!("toplevel wm_capabilities: {caps:?}");
        }
    }
    let mut xdg_toplevel_listener = XDGToplevelListenerInstance;
    xdg_toplevel.add_listener(&mut xdg_toplevel_listener);
    xdg_toplevel.set_app_id(c"io.ct2.bedrock.exmaples.wayland");
    xdg_toplevel.set_title(c"Bedrock Examples (Wayland Native)");

    surface.commit();

    let vk_instance = Rc::new(
        br::InstanceObject::new(&br::InstanceCreateInfo::new(
            &br::ApplicationInfo::new(c"Bedrock Examples Wayland Native", (0, 1, 0), c"", (0, 1, 0))
                .api_version(1, 3, 0),
            &[c"VK_LAYER_KHRONOS_validation".into()],
            &[
                c"VK_KHR_surface".into(),
                c"VK_KHR_wayland_surface".into(),
                c"VK_EXT_debug_utils".into(),
            ],
        ))
        .unwrap(),
    );
    let _vk_debugger = br::DebugUtilsMessengerObject::new(
        vk_instance.clone(),
        &br::DebugUtilsMessengerCreateInfo::new(
            br::vk::VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT
                | br::vk::VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT,
            br::vk::VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT
                | br::vk::VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT
                | br::vk::VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT,
            vk_debug_msg,
        ),
    )
    .unwrap();
    let vk_pdev = vk_instance.iter_physical_devices().unwrap().next().unwrap();
    let vk_surface = unsafe {
        br::SurfaceObject::new(
            &vk_pdev,
            &br::vk::VkWaylandSurfaceCreateInfoKHR::new(
                display.as_proxy_ptr_mut() as _,
                surface.as_proxy_ptr_mut() as _,
            ),
        )
        .unwrap()
    };

    let device_memory_properties = vk_pdev.memory_properties();
    let vk_graphics_queue_family_index = vk_pdev
        .queue_family_properties()
        .find_matching_index(br::QueueFlags::GRAPHICS)
        .unwrap();
    let vk_device = Rc::new(
        br::DeviceObject::new(
            &vk_pdev,
            &br::DeviceCreateInfo::new(
                &[br::DeviceQueueCreateInfo::new(vk_graphics_queue_family_index, &[0.0])],
                &[],
                &[c"VK_KHR_swapchain".into()],
            )
            .with_next(
                &br::vk::VkPhysicalDeviceFeatures2KHR::new(Default::default())
                    .with_next(&mut br::vk::VkPhysicalDeviceSynchronization2FeaturesKHR::new(true)),
            ),
        )
        .unwrap(),
    );
    let mut vk_queue = vk_device.clone().queue(vk_graphics_queue_family_index, 0);

    let surface_props = vk_pdev.surface_capabilities(&vk_surface).unwrap();
    let presentation_modes = vk_pdev.surface_present_modes(&vk_surface).unwrap();
    let vk_swapchain = br::SwapchainBuilder::new(
        vk_surface,
        2,
        br::vk::VkSurfaceFormatKHR {
            format: br::vk::VK_FORMAT_R8G8B8A8_SRGB,
            colorSpace: br::vk::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
        },
        br::vk::VkExtent2D {
            width: 640,
            height: 480,
        },
        br::ImageUsageFlags::COLOR_ATTACHMENT,
    )
    .pre_transform(surface_props.currentTransform)
    .composite_alpha(
        if surface_props
            .supported_composite_alpha()
            .has(br::CompositeAlphaFlags::OPAQUE)
        {
            br::vk::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR
        } else {
            br::vk::VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR
        },
    )
    .present_mode(*presentation_modes.first().unwrap())
    .create(vk_device.clone())
    .unwrap();
    let mut vk_swapchain = Rc::new(vk_swapchain);
    let backbuffers = vk_swapchain
        .get_images()
        .unwrap()
        .into_iter()
        .map(|bb| bb.clone_parent())
        .collect::<Vec<_>>();

    let renderpass = br::RenderPassObject::new(
        vk_device.clone(),
        &br::RenderPassCreateInfo2::new(
            &[br::AttachmentDescription2::new(br::vk::VK_FORMAT_R8G8B8A8_SRGB)
                .with_layout_to(br::ImageLayout::PresentSrc.from_undefined())
                .color_memory_op(br::LoadOp::Clear, br::StoreOp::Store)],
            &[br::SubpassDescription2::new()
                .colors(&[br::AttachmentReference2::color(0, br::ImageLayout::ColorAttachmentOpt)])],
            &[
                br::SubpassDependency2::new(br::SubpassIndex::Internal(0), br::SubpassIndex::External)
                    .by_region()
                    .of_memory(br::AccessFlags::COLOR_ATTACHMENT.write, br::AccessFlags::MEMORY.read)
                    .of_execution(
                        br::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                        br::PipelineStageFlags::ALL_COMMANDS,
                    ),
            ],
        ),
    )
    .unwrap();

    let backbuffer_views = backbuffers
        .iter()
        .map(|bb| {
            bb.subresource_range(br::AspectMask::COLOR, 0..1, 0..1)
                .view_builder()
                .create()
        })
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let framebuffers = backbuffer_views
        .iter()
        .map(|bb| {
            br::FramebufferObject::new(
                vk_device.clone(),
                &br::FramebufferCreateInfo::new(&renderpass, &[bb.as_transparent_ref()], 640, 480),
            )
        })
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let rect = br::vk::VkExtent2D {
        width: 640,
        height: 480,
    }
    .into_rect(br::vk::VkOffset2D::ZERO);
    let viewport = rect.make_viewport(0.0..1.0);

    let dsl_ub1 = br::DescriptorSetLayoutObject::new(
        &vk_device,
        &br::DescriptorSetLayoutCreateInfo::new(&[br::DescriptorType::UniformBuffer.make_binding(0, 1)]),
    )
    .unwrap();

    let vsh = br::ShaderModuleObject::new(
        &vk_device,
        &br::ShaderModuleCreateInfo::new(&read_spv_binary("./shaders/triangle.vspv").unwrap()),
    )
    .unwrap();
    let fsh = br::ShaderModuleObject::new(
        &vk_device,
        &br::ShaderModuleCreateInfo::new(&read_spv_binary("./shaders/triangle.fspv").unwrap()),
    )
    .unwrap();
    let vi_bindings = [br::vk::VkVertexInputBindingDescription::per_vertex_typed::<Vertex>(0)];
    let vi_attrs = [
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
            offset: core::mem::offset_of!(Vertex, col) as _,
        },
    ];
    let pl = br::PipelineLayoutObject::new(
        &vk_device,
        &br::PipelineLayoutCreateInfo::new(
            &[dsl_ub1.as_transparent_ref()],
            &[br::vk::VkPushConstantRange::for_type::<[f32; 2]>(
                br::vk::VK_SHADER_STAGE_VERTEX_BIT,
                0,
            )],
        ),
    )
    .unwrap();
    let shader_stages = &[
        vsh.with_entry_point(c"main").on_stage(br::ShaderStage::Vertex),
        fsh.with_entry_point(c"main").on_stage(br::ShaderStage::Fragment),
    ];
    let mut pipeline = br::NonDerivedGraphicsPipelineBuilder::new(
        &pl,
        renderpass.subpass(0),
        br::VertexProcessingStages::new(
            shader_stages,
            &vi_bindings,
            &vi_attrs,
            br::vk::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
        ),
    );
    let viewports = &[viewport];
    let scissors = &[rect];
    pipeline
        .multisample_state(Some(br::MultisampleState::new()))
        .color_blend_state(br::ColorBlendState::new(
            None,
            &[br::vk::VkPipelineColorBlendAttachmentState::NOBLEND],
            [0.0; 4],
        ))
        .viewport_state(br::ViewportState::new(viewports, scissors));
    let pipeline = pipeline
        .create(
            &vk_device,
            None::<&br::PipelineCacheObject<Rc<br::DeviceObject<&Rc<br::InstanceObject>>>>>,
        )
        .unwrap();

    let vertex_buffer_offset = 0;
    let uniform_buffer_offset = 256;
    let total_buffer_size = uniform_buffer_offset + core::mem::size_of::<ObjectParameters>();
    let mut device_buffer = br::BufferObject::new(
        &vk_device,
        &br::BufferCreateInfo::new(
            total_buffer_size,
            br::BufferUsage::VERTEX_BUFFER.uniform_buffer().transfer_dest(),
        ),
    )
    .unwrap();
    let mut staging_buffer = br::BufferObject::new(
        &vk_device,
        &br::BufferCreateInfo::new(total_buffer_size, br::BufferUsage::TRANSFER_SRC),
    )
    .unwrap();
    let device_memory_req = device_buffer.requirements();
    let staging_memory_req = staging_buffer.requirements();
    let device_memory_type_index = device_memory_properties
        .find_device_local_index(device_memory_req.memoryTypeBits)
        .unwrap();
    let staging_memory_type_index = device_memory_properties
        .find_host_visible_index(staging_memory_req.memoryTypeBits)
        .unwrap();
    let device_memory = br::DeviceMemoryRequest::allocate(device_memory_req.size as _, device_memory_type_index)
        .execute(&vk_device)
        .unwrap();
    let mut staging_memory = br::DeviceMemoryRequest::allocate(staging_memory_req.size as _, staging_memory_type_index)
        .execute(&vk_device)
        .unwrap();
    device_buffer.bind(&device_memory, 0).unwrap();
    staging_buffer.bind(&staging_memory, 0).unwrap();
    unsafe {
        let ptr = staging_memory.map(0..total_buffer_size).unwrap();

        ptr.clone_from_slice_at(
            vertex_buffer_offset,
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
        ptr.get_mut::<ObjectParameters>(uniform_buffer_offset).rot = 0.0;

        staging_memory.unmap();
    }

    let mut init_cp = br::CommandPoolObject::new(
        &vk_device,
        &br::CommandPoolCreateInfo::new(vk_graphics_queue_family_index).transient(),
    )
    .unwrap();
    let [mut init_cb] = br::CommandBufferObject::alloc_array(
        &vk_device,
        &br::CommandBufferFixedCountAllocateInfo::<'_, 1>::new(&mut init_cp, br::CommandBufferLevel::Primary),
    )
    .unwrap();
    unsafe { init_cb.begin_once(&vk_device).unwrap() }
        .copy_buffer(
            &staging_buffer,
            &device_buffer,
            &[br::BufferCopy::mirror(0, total_buffer_size as _)],
        )
        .pipeline_barrier_2(&br::DependencyInfo::new(
            &[br::MemoryBarrier2::new()
                .from(br::PipelineStageFlags2::COPY, br::AccessFlags2::TRANSFER.write)
                .to(
                    br::PipelineStageFlags2::VERTEX_INPUT | br::PipelineStageFlags2::VERTEX_SHADER,
                    br::AccessFlags2::VERTEX_ATTRIBUTE_READ | br::AccessFlags2::UNIFORM_READ,
                )],
            &[],
            &[],
        ))
        .end()
        .unwrap();
    vk_queue
        .submit2(
            &[br::SubmitInfo2::new(
                &[],
                &[br::CommandBufferSubmitInfo::new(&init_cb)],
                &[],
            )],
            None,
        )
        .unwrap();
    vk_queue.wait().unwrap();

    let mut dp = br::DescriptorPoolObject::new(
        &vk_device,
        &br::DescriptorPoolCreateInfo::new(1, &[br::DescriptorType::UniformBuffer.make_size(1)]),
    )
    .unwrap();
    let [object_descriptor] = dp.alloc_array(&[dsl_ub1.as_transparent_ref()]).unwrap();
    vk_device.update_descriptor_sets(
        &[object_descriptor
            .binding_at(0)
            .write(br::DescriptorContents::uniform_buffer(
                &device_buffer,
                uniform_buffer_offset as u64
                    ..uniform_buffer_offset as u64 + core::mem::size_of::<ObjectParameters>() as u64,
            ))],
        &[],
    );

    let mut update_cp = br::CommandPoolObject::new(
        &vk_device,
        &br::CommandPoolCreateInfo::new(vk_graphics_queue_family_index),
    )
    .unwrap();
    let [mut update_cb] = br::CommandBufferObject::alloc_array(
        &vk_device,
        &br::CommandBufferFixedCountAllocateInfo::<'_, 1>::new(&mut update_cp, br::CommandBufferLevel::Primary),
    )
    .unwrap();
    unsafe { update_cb.begin(&vk_device).unwrap() }
        .copy_buffer(
            &staging_buffer,
            &device_buffer,
            &[br::BufferCopy::mirror_data::<ObjectParameters>(
                uniform_buffer_offset as _,
            )],
        )
        .pipeline_barrier_2(&br::DependencyInfo::new(
            &[br::MemoryBarrier2::new()
                .from(br::PipelineStageFlags2::COPY, br::AccessFlags2::TRANSFER.write)
                .to(br::PipelineStageFlags2::VERTEX_SHADER, br::AccessFlags2::UNIFORM_READ)],
            &[],
            &[],
        ))
        .end()
        .unwrap();

    let mut cp = br::CommandPoolObject::new(
        vk_device.clone(),
        &br::CommandPoolCreateInfo::new(vk_graphics_queue_family_index),
    )
    .unwrap();
    let mut cb = br::CommandBufferObject::alloc(
        vk_device.clone(),
        &br::CommandBufferAllocateInfo::new(&mut cp, framebuffers.len() as _, br::CommandBufferLevel::Primary),
    )
    .unwrap();
    for (cb, fb) in cb.iter_mut().zip(framebuffers.iter()) {
        unsafe { cb.begin(&vk_device).unwrap() }
            .begin_render_pass_2(
                &br::RenderPassBeginInfo::new(
                    &renderpass,
                    fb,
                    br::vk::VkExtent2D {
                        width: 640,
                        height: 480,
                    }
                    .into_rect(br::vk::VkOffset2D::ZERO),
                    &[br::ClearValue::color_f32([0.0, 0.0, 0.0, 1.0])],
                ),
                &br::vk::VkSubpassBeginInfo::new(br::vk::VK_SUBPASS_CONTENTS_INLINE),
            )
            .bind_graphics_pipeline(&pipeline)
            .push_constant(&pl, br::vk::VK_SHADER_STAGE_VERTEX_BIT, 0, &[640.0f32, 480.0])
            .bind_graphics_descriptor_sets(&pl, 0, &[object_descriptor], &[])
            .bind_vertex_buffers(0, &[device_buffer.as_transparent_ref()], &[vertex_buffer_offset as _])
            .draw(3, 1, 0, 0)
            .end_render_pass_2(&br::vk::VkSubpassEndInfo::new())
            .end()
            .unwrap();
    }

    let mut fence = br::FenceObject::new(vk_device.clone(), &br::FenceCreateInfo::new(0)).unwrap();
    let bb_index = vk_swapchain
        .acquire_next(None, br::CompletionHandlerMut::Host(fence.as_transparent_ref_mut()))
        .unwrap();
    fence.wait().unwrap();
    fence.reset().unwrap();
    vk_queue
        .submit2(
            &[br::SubmitInfo2::new(
                &[],
                &[br::CommandBufferSubmitInfo::new(&cb[bb_index as usize])],
                &[],
            )],
            Some(fence.as_transparent_ref_mut()),
        )
        .unwrap();
    fence.wait().unwrap();
    fence.reset().unwrap();
    vk_queue
        .present(br::PresentInfo::new(
            &[] as &[br::VkHandleRef<br::vk::VkSemaphore>],
            &[vk_swapchain.as_transparent_ref()],
            &[bb_index],
        ))
        .unwrap();
    vk_queue.wait().unwrap();

    struct CallbackListenerInstance<'s> {
        surface: &'s mut WlSurface,
        swapchain: Rc<
            br::SurfaceSwapchainObject<
                Rc<br::DeviceObject<&'s Rc<br::InstanceObject>>>,
                br::SurfaceObject<&'s Rc<br::InstanceObject>>,
            >,
        >,
        fence: br::FenceObject<Rc<br::DeviceObject<&'s Rc<br::InstanceObject>>>>,
        queue: br::QueueObject<Rc<br::DeviceObject<&'s Rc<br::InstanceObject>>>>,
        command_buffers: &'s [br::CommandBufferObject<Rc<br::DeviceObject<&'s Rc<br::InstanceObject>>>>],
        update_command_buffer: br::CommandBufferObject<&'s Rc<br::DeviceObject<&'s Rc<br::InstanceObject>>>>,
        staging_memory: br::DeviceMemoryObject<&'s Rc<br::DeviceObject<&'s Rc<br::InstanceObject>>>>,
        uniform_buffer_offset: usize,
        staging_memory_size: usize,
        rot: f32,
        last_time: c_uint,
    }
    impl WlCallbackListener for CallbackListenerInstance<'_> {
        fn done(&mut self, obj: &mut WlCallback, time: c_uint) {
            obj.manual_destroy();
            let cb = self.surface.frame();
            cb.add_listener(self);

            if self.last_time != 0 {
                self.rot += 180.0f32.to_radians() * (time - self.last_time) as f32 / 1000.0;
            }
            self.last_time = time;

            unsafe {
                let ptr = self.staging_memory.map(0..self.staging_memory_size).unwrap();

                ptr.get_mut::<ObjectParameters>(self.uniform_buffer_offset).rot = self.rot;

                self.staging_memory.unmap();
            }

            let bb_index = self
                .swapchain
                .acquire_next(
                    None,
                    br::CompletionHandlerMut::Host(self.fence.as_transparent_ref_mut()),
                )
                .unwrap();
            self.fence.wait().unwrap();
            self.fence.reset().unwrap();
            self.queue
                .submit2(
                    &[br::SubmitInfo2::new(
                        &[],
                        &[
                            br::CommandBufferSubmitInfo::new(&self.update_command_buffer),
                            br::CommandBufferSubmitInfo::new(&self.command_buffers[bb_index as usize]),
                        ],
                        &[],
                    )],
                    Some(self.fence.as_transparent_ref_mut()),
                )
                .unwrap();
            self.fence.wait().unwrap();
            self.fence.reset().unwrap();
            self.queue
                .present(br::PresentInfo::new(
                    &[] as &[br::VkHandleRef<br::vk::VkSemaphore>],
                    &[self.swapchain.as_transparent_ref()],
                    &[bb_index],
                ))
                .unwrap();
            self.queue.wait().unwrap();
        }
    }
    let mut callback_listener = CallbackListenerInstance {
        surface: &mut surface,
        swapchain: vk_swapchain,
        fence,
        queue: vk_queue,
        command_buffers: &cb,
        update_command_buffer: update_cb,
        staging_memory,
        staging_memory_size: total_buffer_size,
        uniform_buffer_offset,
        rot: 0.0,
        last_time: 0,
    };
    let cb = callback_listener.surface.frame();
    cb.add_listener(&mut callback_listener);

    while display.dispatch() != -1 {}

    drop(callback_listener);
    drop(framebuffers);
    drop(backbuffer_views);
    drop(renderpass);
    drop(backbuffers);
    drop(xdg_toplevel);
    drop(xdg_surface);
    drop(surface);
    drop(display);
}

extern "system" fn vk_debug_msg(
    _message_severity: br::vk::VkDebugUtilsMessageSeverityFlagBitsEXT,
    _message_types: br::vk::VkDebugUtilsMessageTypeFlagsEXT,
    callback_data: *const br::vk::VkDebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut c_void,
) -> br::vk::VkBool32 {
    let msg = unsafe { core::ffi::CStr::from_ptr((*callback_data).pMessage).to_str().unwrap() };

    eprintln!("*vk_debug_msg* {msg}");

    true as _
}

#[repr(transparent)]
pub struct WlDisplayConnection(core::ptr::NonNull<WlDisplay>);
impl Drop for WlDisplayConnection {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            wl_display_disconnect(self.0.as_ptr() as _);
        }
    }
}
impl Deref for WlDisplayConnection {
    type Target = WlDisplay;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}
impl DerefMut for WlDisplayConnection {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}
impl WlDisplayConnection {
    #[inline]
    pub fn new(name: Option<&core::ffi::CStr>) -> Self {
        let ptr = unsafe { wl_display_connect(name.map_or_else(core::ptr::null, |x| x.as_ptr())) };

        Self(core::ptr::NonNull::new(ptr as *mut WlDisplay).unwrap())
    }
}

#[repr(transparent)]
pub struct WlDisplay(wl_proxy);
impl WlDisplay {
    #[inline(always)]
    fn as_proxy_ptr_mut(&mut self) -> *mut wl_proxy {
        self as *mut _ as _
    }

    #[inline]
    pub fn get_registry(&mut self) -> &mut WlRegistry {
        unsafe {
            let ptr = wl_proxy_marshal_flags(
                self as *mut _ as _,
                WL_DISPLAY_GET_REGISTRY,
                &wl_registry_interface,
                wl_proxy_get_version(self as *mut _ as _),
                0,
                core::ptr::null_mut::<wl_proxy>(),
            );

            &mut *(ptr as *mut WlRegistry)
        }
    }

    #[inline]
    pub fn roundtrip(&mut self) -> c_int {
        unsafe { wl_display_roundtrip(self as *mut _ as _) }
    }

    #[inline]
    pub fn dispatch(&mut self) -> c_int {
        unsafe { wl_display_dispatch(self as *mut _ as _) }
    }
}

pub trait RegistryListener {
    fn global(&mut self, obj: &mut WlRegistry, name: u32, interface: &core::ffi::CStr, version: u32);
    fn global_remove(&mut self, obj: &mut WlRegistry, name: u32);
}

#[repr(transparent)]
pub struct WlRegistry(wl_proxy);
impl WlRegistry {
    pub fn add_listener<L: RegistryListener + 'static>(&mut self, listener: &mut L) {
        extern "C" fn global<L: RegistryListener + 'static>(
            data: *mut c_void,
            obj: *mut wl_proxy,
            name: u32,
            interface: *const c_char,
            version: u32,
        ) {
            unsafe {
                (&mut *(data as *mut L)).global(
                    &mut *(obj as *mut WlRegistry),
                    name,
                    core::ffi::CStr::from_ptr(interface),
                    version,
                );
            }
        }
        extern "C" fn global_remove<L: RegistryListener + 'static>(data: *mut c_void, obj: *mut wl_proxy, name: u32) {
            unsafe {
                (&mut *(data as *mut L)).global_remove(&mut *(obj as *mut WlRegistry), name);
            }
        }
        #[repr(C)]
        struct ListenerFunctionPointers {
            global: extern "C" fn(*mut c_void, *mut wl_proxy, u32, *const c_char, u32),
            global_remove: extern "C" fn(*mut c_void, *mut wl_proxy, u32),
        }
        let fps: &'static ListenerFunctionPointers = &ListenerFunctionPointers {
            global: global::<L>,
            global_remove: global_remove::<L>,
        };

        let res = unsafe {
            wl_proxy_add_listener(
                self as *mut _ as _,
                fps as *const ListenerFunctionPointers as _,
                listener as *mut L as _,
            )
        };
        assert!(res >= 0, "wl_proxy_add_listener failed: {res}");
    }

    #[inline]
    pub fn bind(&mut self, name: u32, interface: &wl_interface, version: u32) -> *mut wl_proxy {
        unsafe {
            wl_proxy_marshal_flags(
                self as *mut _ as _,
                WL_REGISTRY_BIND,
                interface as *const _,
                version,
                0,
                name,
                interface.name,
                version,
                core::ptr::null_mut::<wl_proxy>(),
            )
        }
    }
}

#[allow(non_camel_case_types)]
pub enum wl_display {}
#[allow(non_camel_case_types)]
pub enum wl_proxy {}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct wl_message {
    pub name: *const c_char,
    pub signature: *const c_char,
    pub types: *const *const wl_interface,
}
unsafe impl Sync for wl_message {}
impl wl_message {
    const fn new(name: &'static CStr, signature: &'static CStr, types: &'static [*const wl_interface]) -> Self {
        Self {
            name: name.as_ptr(),
            signature: signature.as_ptr(),
            types: types.as_ptr(),
        }
    }
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct wl_interface {
    pub name: *const c_char,
    pub version: c_int,
    pub method_count: c_int,
    pub methods: *const wl_message,
    pub event_count: c_int,
    pub events: *const wl_message,
}
unsafe impl Sync for wl_interface {}
impl wl_interface {
    const fn new(
        name: &'static CStr,
        version: c_int,
        methods: &'static [wl_message],
        events: &'static [wl_message],
    ) -> Self {
        Self {
            name: name.as_ptr(),
            version,
            method_count: methods.len() as _,
            methods: methods.as_ptr(),
            event_count: events.len() as _,
            events: events.as_ptr(),
        }
    }
}

static XDG_TOPLEVEL_INTERFACE: wl_interface = wl_interface::new(
    c"xdg_toplevel",
    6,
    &[
        wl_message::new(c"destroy", c"", &[]),
        wl_message::new(c"set_parent", c"?o", &[&XDG_TOPLEVEL_INTERFACE as *const _]),
        wl_message::new(c"set_title", c"s", &[core::ptr::null()]),
        wl_message::new(c"set_app_id", c"s", &[core::ptr::null()]),
        wl_message::new(
            c"show_window_menu",
            c"ouii",
            &[
                unsafe { &wl_seat_interface as *const _ },
                core::ptr::null(),
                core::ptr::null(),
                core::ptr::null(),
            ],
        ),
        wl_message::new(
            c"move",
            c"ou",
            &[unsafe { &wl_seat_interface as *const _ }, core::ptr::null()],
        ),
        wl_message::new(
            c"resize",
            c"ouu",
            &[
                unsafe { &wl_seat_interface as *const _ },
                core::ptr::null(),
                core::ptr::null(),
            ],
        ),
        wl_message::new(c"set_max_size", c"ii", &[core::ptr::null(); 2]),
        wl_message::new(c"set_min_size", c"ii", &[core::ptr::null(); 2]),
        wl_message::new(c"set_maximized", c"", &[]),
        wl_message::new(c"unset_maximized", c"", &[]),
        wl_message::new(c"set_fullscreen", c"?o", &[unsafe { &wl_output_interface as *const _ }]),
        wl_message::new(c"unset_fullscreen", c"", &[]),
        wl_message::new(c"set_minimized", c"", &[]),
    ],
    &[
        wl_message::new(c"configure", c"iia", &[core::ptr::null(); 3]),
        wl_message::new(c"close", c"", &[]),
        wl_message::new(c"configure_bounds", c"4ii", &[core::ptr::null(); 2]),
        wl_message::new(c"wm_capabilities", c"5a", &[core::ptr::null()]),
    ],
);
pub const XDG_TOPLEVEL_DESTROY: u32 = 0;
pub const XDG_TOPLEVEL_SET_TITLE: u32 = 2;
pub const XDG_TOPLEVEL_SET_APP_ID: u32 = 3;

pub trait XDGToplevelListener {
    fn configure(&mut self, obj: &mut XDGToplevel, width: c_int, height: c_int, states: *mut wl_array);
    fn close(&mut self, obj: &mut XDGToplevel);
    fn configure_bounds(&mut self, obj: &mut XDGToplevel, width: c_int, height: c_int);
    fn wm_capabilities(&mut self, obj: &mut XDGToplevel, capabilities: *mut wl_array);
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum XDGToplevelResizeEdge {
    None = 0,
    Top = 1,
    Bottom = 2,
    Left = 4,
    TopLeft = 5,
    BottomLeft = 6,
    Right = 8,
    TopRight = 9,
    BottomRight = 10,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum XDGToplevelState {
    Maximized = 1,
    Fullscreen = 2,
    Resizing = 3,
    Activated = 4,
    /// since v2
    TiledLeft = 5,
    /// since v2
    TiledRight = 6,
    /// since v2
    TiledTop = 7,
    /// since v2
    TiledBottom = 8,
    /// since v6
    Suspended = 9,
}

/// since v5
#[repr(C)]
#[derive(Clone, Copy)]
pub enum XDGToplevelWMCapabilities {
    WindowMenu = 1,
    Maximize = 2,
    Fullscreen = 3,
    Minimize = 4,
}

static XDG_POPUP_INTERFACE: wl_interface = wl_interface::new(c"xdg_popup", 6, &[], &[]);

static XDG_POSITIONER_INTERFACE: wl_interface = wl_interface::new(c"xdg_positioner", 6, &[], &[]);

static XDG_SURFACE_INTERFACE: wl_interface = wl_interface::new(
    c"xdg_surface",
    6,
    &[
        wl_message::new(c"destroy", c"", &[]),
        wl_message::new(c"get_toplevel", c"n", &[&XDG_TOPLEVEL_INTERFACE as *const _]),
        wl_message::new(
            c"get_popup",
            c"n?oo",
            &[
                &XDG_POPUP_INTERFACE as *const _,
                &XDG_SURFACE_INTERFACE as *const _,
                &XDG_POSITIONER_INTERFACE as *const _,
            ],
        ),
        wl_message::new(c"set_window_geometry", c"iiii", &[core::ptr::null(); 4]),
        wl_message::new(c"ack_configure", c"u", &[core::ptr::null()]),
    ],
    &[wl_message::new(c"configure", c"u", &[core::ptr::null()])],
);
pub const XDG_SURFACE_DESTROY: u32 = 0;
pub const XDG_SURFACE_GET_TOPLEVEL: u32 = 1;
pub const XDG_SURFACE_ACK_CONFIGURE: u32 = 4;

pub trait XDGSurfaceListener {
    fn configure(&mut self, obj: &mut XDGSurface, serial: c_uint);
}

static XDG_WM_BASE_INTERFACE: wl_interface = wl_interface::new(
    c"xdg_wm_base",
    6,
    &[
        wl_message::new(c"destroy", c"", &[]),
        wl_message::new(c"create_positioner", c"n", &[&XDG_POSITIONER_INTERFACE as *const _]),
        wl_message::new(
            c"get_xdg_surface",
            c"no",
            &[&XDG_SURFACE_INTERFACE as *const _, unsafe {
                &wl_surface_interface as *const _
            }],
        ),
        wl_message::new(c"pong", c"u", &[core::ptr::null()]),
    ],
    &[],
);
pub const XDG_WM_BASE_DESTROY: u32 = 0;
pub const XDG_WM_BASE_GET_XDG_SURFACE: u32 = 2;

pub const WL_DISPLAY_GET_REGISTRY: u32 = 1;
pub const WL_REGISTRY_BIND: u32 = 0;
pub const WL_COMPOSITOR_CREATE_SURFACE: u32 = 0;
pub const WL_SURFACE_DESTROY: u32 = 0;
pub const WL_SURFACE_FRAME: u32 = 3;
pub const WL_SURFACE_COMMIT: u32 = 6;

#[repr(transparent)]
pub struct WlCompositor(wl_proxy);
impl WlCompositor {
    #[inline]
    pub fn create_surface(&mut self) -> OwnedWlSurface {
        unsafe {
            let ptr = wl_proxy_marshal_flags(
                self as *mut _ as _,
                WL_COMPOSITOR_CREATE_SURFACE,
                &wl_surface_interface as *const _,
                wl_proxy_get_version(self as *mut _ as _),
                0,
                core::ptr::null_mut::<wl_proxy>(),
            );

            OwnedWlSurface(core::ptr::NonNull::new(ptr as *mut WlSurface).unwrap())
        }
    }
}

#[repr(transparent)]
pub struct OwnedWlSurface(pub core::ptr::NonNull<WlSurface>);
impl Drop for OwnedWlSurface {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            wl_proxy_marshal_flags(
                self.0.as_ptr() as _,
                WL_SURFACE_DESTROY,
                core::ptr::null(),
                wl_proxy_get_version(self.0.as_ptr() as _),
                WL_MARSHAL_FLAG_DESTROY,
            );
        }
    }
}
impl Deref for OwnedWlSurface {
    type Target = WlSurface;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}
impl DerefMut for OwnedWlSurface {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

#[repr(transparent)]
pub struct WlSurface(wl_proxy);
impl WlSurface {
    #[inline]
    pub fn as_proxy_ptr_mut(&mut self) -> *mut wl_proxy {
        self as *mut _ as _
    }

    #[inline]
    pub fn commit(&mut self) {
        unsafe {
            wl_proxy_marshal_flags(
                self as *mut _ as _,
                WL_SURFACE_COMMIT,
                core::ptr::null(),
                wl_proxy_get_version(self as *mut _ as _),
                0,
            );
        }
    }

    #[inline]
    pub fn frame<'a>(&mut self) -> &'a mut WlCallback {
        unsafe {
            let ptr = wl_proxy_marshal_flags(
                self as *mut _ as _,
                WL_SURFACE_FRAME,
                &wl_callback_interface,
                wl_proxy_get_version(self as *mut _ as _),
                0,
                core::ptr::null_mut::<wl_proxy>(),
            );

            &mut *(ptr as *mut WlCallback)
        }
    }
}

#[repr(transparent)]
pub struct OwnedXDGWmBase(pub core::ptr::NonNull<XDGWmBase>);
impl Drop for OwnedXDGWmBase {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            wl_proxy_marshal_flags(
                self.0.as_ptr() as _,
                XDG_WM_BASE_DESTROY,
                core::ptr::null(),
                wl_proxy_get_version(self.0.as_ptr() as _),
                WL_MARSHAL_FLAG_DESTROY,
            );
        }
    }
}
impl Deref for OwnedXDGWmBase {
    type Target = XDGWmBase;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}
impl DerefMut for OwnedXDGWmBase {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

#[repr(transparent)]
pub struct XDGWmBase(wl_proxy);
impl XDGWmBase {
    #[inline]
    pub fn get_xdg_surface(&mut self, surface: &mut WlSurface) -> OwnedXDGSurface {
        unsafe {
            let ptr = wl_proxy_marshal_flags(
                self as *mut _ as _,
                XDG_WM_BASE_GET_XDG_SURFACE,
                &XDG_SURFACE_INTERFACE,
                wl_proxy_get_version(self as *mut _ as _),
                0,
                core::ptr::null_mut::<wl_proxy>(),
                surface as *mut WlSurface as *mut wl_proxy,
            );

            OwnedXDGSurface::new(ptr).unwrap()
        }
    }
}

#[repr(transparent)]
pub struct OwnedXDGSurface(pub core::ptr::NonNull<XDGSurface>);
impl Drop for OwnedXDGSurface {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            wl_proxy_marshal_flags(
                self.as_proxy_ptr_mut(),
                XDG_SURFACE_DESTROY,
                core::ptr::null(),
                wl_proxy_get_version(self.as_proxy_ptr_mut()),
                WL_MARSHAL_FLAG_DESTROY,
            );
        }
    }
}
impl OwnedXDGSurface {
    #[inline(always)]
    pub unsafe fn new(proxy: *mut wl_proxy) -> Option<Self> {
        core::ptr::NonNull::new(proxy as *mut XDGSurface).map(Self)
    }
}
impl Deref for OwnedXDGSurface {
    type Target = XDGSurface;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}
impl DerefMut for OwnedXDGSurface {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

#[repr(transparent)]
pub struct XDGSurface(wl_proxy);
impl XDGSurface {
    #[inline(always)]
    fn as_proxy_ptr_mut(&mut self) -> *mut wl_proxy {
        self as *mut Self as _
    }

    #[inline]
    pub fn add_listener<L: XDGSurfaceListener + 'static>(&mut self, listener: &mut L) {
        extern "C" fn configure<L: XDGSurfaceListener + 'static>(
            data: *mut c_void,
            obj: *mut wl_proxy,
            serial: c_uint,
        ) {
            unsafe {
                (&mut *(data as *mut L)).configure(&mut *(obj as *mut XDGSurface), serial);
            }
        }
        #[repr(C)]
        struct ListenerFunctionPointers {
            configure: extern "C" fn(*mut c_void, *mut wl_proxy, c_uint),
        }
        let listener_function_pointers: &'static ListenerFunctionPointers = &ListenerFunctionPointers {
            configure: configure::<L>,
        };

        let res = unsafe {
            wl_proxy_add_listener(
                self.as_proxy_ptr_mut(),
                listener_function_pointers as *const ListenerFunctionPointers as _,
                listener as *mut L as _,
            )
        };
        assert!(res >= 0, "wl_proxy_add_listener failed: {res}");
    }

    #[inline]
    pub fn get_toplevel(&mut self) -> OwnedXDGToplevel {
        unsafe {
            let ptr = wl_proxy_marshal_flags(
                self.as_proxy_ptr_mut(),
                XDG_SURFACE_GET_TOPLEVEL,
                &XDG_TOPLEVEL_INTERFACE,
                wl_proxy_get_version(self.as_proxy_ptr_mut()),
                0,
                core::ptr::null_mut::<wl_proxy>(),
            );

            OwnedXDGToplevel(core::ptr::NonNull::new(ptr as *mut XDGToplevel).unwrap())
        }
    }

    #[inline]
    pub fn ack_configure(&mut self, serial: c_uint) {
        unsafe {
            wl_proxy_marshal_flags(
                self.as_proxy_ptr_mut(),
                XDG_SURFACE_ACK_CONFIGURE,
                core::ptr::null(),
                wl_proxy_get_version(self.as_proxy_ptr_mut()),
                0,
                serial,
            );
        }
    }
}

#[repr(transparent)]
pub struct OwnedXDGToplevel(pub core::ptr::NonNull<XDGToplevel>);
impl Drop for OwnedXDGToplevel {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            wl_proxy_marshal_flags(
                self.0.as_ptr() as _,
                XDG_TOPLEVEL_DESTROY,
                core::ptr::null(),
                wl_proxy_get_version(self.0.as_ptr() as _),
                WL_MARSHAL_FLAG_DESTROY,
            );
        }
    }
}
impl Deref for OwnedXDGToplevel {
    type Target = XDGToplevel;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}
impl DerefMut for OwnedXDGToplevel {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

#[repr(transparent)]
pub struct XDGToplevel(wl_proxy);
impl XDGToplevel {
    #[inline]
    pub fn add_listener<L: XDGToplevelListener + 'static>(&mut self, listener: &mut L) {
        extern "C" fn configure<L: XDGToplevelListener + 'static>(
            data: *mut c_void,
            obj: *mut wl_proxy,
            width: c_int,
            height: c_int,
            states: *mut wl_array,
        ) {
            unsafe {
                (&mut *(data as *mut L)).configure(&mut *(obj as *mut XDGToplevel), width, height, states);
            }
        }
        extern "C" fn close<L: XDGToplevelListener + 'static>(data: *mut c_void, obj: *mut wl_proxy) {
            unsafe {
                (&mut *(data as *mut L)).close(&mut *(obj as *mut XDGToplevel));
            }
        }
        extern "C" fn configure_bounds<L: XDGToplevelListener + 'static>(
            data: *mut c_void,
            obj: *mut wl_proxy,
            width: c_int,
            height: c_int,
        ) {
            unsafe {
                (&mut *(data as *mut L)).configure_bounds(&mut *(obj as *mut XDGToplevel), width, height);
            }
        }
        extern "C" fn wm_capabilities<L: XDGToplevelListener + 'static>(
            data: *mut c_void,
            obj: *mut wl_proxy,
            capabilities: *mut wl_array,
        ) {
            unsafe {
                (&mut *(data as *mut L)).wm_capabilities(&mut *(obj as *mut XDGToplevel), capabilities);
            }
        }
        #[repr(C)]
        struct ListenerFunctionPointers {
            configure: extern "C" fn(*mut c_void, *mut wl_proxy, c_int, c_int, *mut wl_array),
            close: extern "C" fn(*mut c_void, *mut wl_proxy),
            configure_bounds: extern "C" fn(*mut c_void, *mut wl_proxy, c_int, c_int),
            wm_capabilities: extern "C" fn(*mut c_void, *mut wl_proxy, *mut wl_array),
        }
        let callbacks: &'static ListenerFunctionPointers = &ListenerFunctionPointers {
            configure: configure::<L>,
            close: close::<L>,
            configure_bounds: configure_bounds::<L>,
            wm_capabilities: wm_capabilities::<L>,
        };

        let res = unsafe {
            wl_proxy_add_listener(
                self as *mut _ as _,
                callbacks as *const ListenerFunctionPointers as *const _,
                listener as *mut L as _,
            )
        };
        assert!(res >= 0, "wl_proxy_add_listener failed: {res}");
    }

    #[inline]
    pub fn set_app_id(&mut self, app_id: &core::ffi::CStr) {
        unsafe {
            wl_proxy_marshal_flags(
                self as *mut _ as _,
                XDG_TOPLEVEL_SET_APP_ID,
                core::ptr::null(),
                wl_proxy_get_version(self as *mut _ as _),
                0,
                app_id.as_ptr(),
            );
        }
    }

    #[inline]
    pub fn set_title(&mut self, title: &core::ffi::CStr) {
        unsafe {
            wl_proxy_marshal_flags(
                self as *mut _ as _,
                XDG_TOPLEVEL_SET_TITLE,
                core::ptr::null(),
                wl_proxy_get_version(self as *mut _ as _),
                0,
                title.as_ptr(),
            );
        }
    }
}

pub trait WlCallbackListener {
    fn done(&mut self, obj: &mut WlCallback, time: c_uint);
}

#[repr(transparent)]
pub struct WlCallback(wl_proxy);
impl WlCallback {
    #[inline]
    pub fn manual_destroy(&mut self) {
        unsafe {
            wl_proxy_destroy(self as *mut _ as _);
        }
    }

    #[inline]
    pub fn add_listener<L: WlCallbackListener>(&mut self, listener: &mut L) {
        extern "C" fn done<L: WlCallbackListener>(data: *mut c_void, obj: *mut wl_proxy, time: c_uint) {
            unsafe {
                (&mut *(data as *mut L)).done(&mut *(obj as *mut WlCallback), time);
            }
        }
        #[repr(C)]
        struct ListenerFunctionPointers {
            done: extern "C" fn(*mut c_void, *mut wl_proxy, c_uint),
        }
        let fps: &'static ListenerFunctionPointers = &ListenerFunctionPointers { done: done::<L> };

        let res = unsafe { wl_proxy_add_listener(self as *mut _ as _, fps as *const _ as _, listener as *mut L as _) };
        assert!(res >= 0, "wl_proxy_add_listener failed: {res}");
    }
}

pub const WL_MARSHAL_FLAG_DESTROY: u32 = 1 << 0;

#[link(name = "wayland-client")]
extern "C" {
    pub static wl_seat_interface: wl_interface;
    pub static wl_registry_interface: wl_interface;
    pub static wl_compositor_interface: wl_interface;
    pub static wl_surface_interface: wl_interface;
    pub static wl_output_interface: wl_interface;
    pub static wl_callback_interface: wl_interface;

    pub fn wl_display_connect(name: *const c_char) -> *mut wl_display;
    pub fn wl_display_disconnect(display: *mut wl_display);
    pub fn wl_display_roundtrip(display: *mut wl_display) -> c_int;
    pub fn wl_display_dispatch(display: *mut wl_display) -> c_int;

    pub fn wl_proxy_get_version(proxy: *mut wl_proxy) -> u32;
    pub fn wl_proxy_marshal(proxy: *mut wl_proxy, opcode: u32, ...);
    pub fn wl_proxy_marshal_flags(
        proxy: *mut wl_proxy,
        opcode: u32,
        interface: *const wl_interface,
        version: u32,
        flags: u32,
        ...
    ) -> *mut wl_proxy;
    pub fn wl_proxy_add_listener(proxy: *mut wl_proxy, implementation: *const c_void, data: *mut c_void) -> c_int;
    pub fn wl_proxy_destroy(proxy: *mut wl_proxy);
}

// wayland-util

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct wl_array {
    pub size: usize,
    pub alloc: usize,
    pub data: *mut c_void,
}
impl wl_array {
    pub const unsafe fn as_slice_unchecked<T>(&self) -> &[T] {
        core::slice::from_raw_parts(self.data as *const T, self.size / core::mem::size_of::<T>())
    }
}
