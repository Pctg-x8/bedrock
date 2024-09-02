use bedrock::{
    self as br, CommandBufferMut, CommandPoolMut, Device, Fence, FenceMut, ImageSubresourceSlice, Instance,
    PhysicalDevice, Queue, Swapchain, VulkanStructure,
};
use core::ffi::*;
use std::rc::Rc;

struct WaylandGlobalInterface {
    pub obj: *mut wl_proxy,
    pub version: u32,
}

struct WaylandInterfaces {
    pub compositor: Option<WaylandGlobalInterface>,
    pub xdg_wm_base: Option<WaylandGlobalInterface>,
}

fn main() {
    let con = unsafe { wl_display_connect(core::ptr::null()) };
    if con.is_null() {
        panic!("Failed to connect to wayland server");
    }

    let proxy_version = unsafe { wl_proxy_get_version(con as _) };
    let registry = unsafe {
        wl_proxy_marshal_flags(
            con as _,
            WL_DISPLAY_GET_REGISTRY,
            core::ptr::addr_of!(wl_registry_interface),
            proxy_version,
            0,
            core::ptr::null_mut::<c_void>(),
        )
    };
    if registry.is_null() {
        panic!("Failed to get wayland registry");
    }

    let mut interfaces = WaylandInterfaces {
        compositor: None,
        xdg_wm_base: None,
    };
    let mut registry_listeners = [
        registry_handle_global as *const c_void,
        registry_handle_global_remove as *const c_void,
    ];
    let res = unsafe {
        wl_proxy_add_listener(
            registry,
            registry_listeners.as_mut_ptr(),
            &mut interfaces as *mut _ as _,
        )
    };
    if res < 0 {
        panic!("registry listener adding error");
    }

    unsafe {
        wl_display_roundtrip(con);
    }

    let compositor = interfaces.compositor.expect("no wl_compositor found in a roundtrip");
    let xdg_wm_base = interfaces.xdg_wm_base.expect("no xdg_wm_base found in a roundtrip");
    let surface = unsafe {
        wl_proxy_marshal_flags(
            compositor.obj,
            WL_COMPOSITOR_CREATE_SURFACE,
            core::ptr::addr_of!(wl_surface_interface),
            wl_proxy_get_version(compositor.obj),
            0,
            core::ptr::null_mut::<()>(),
        )
    };
    assert!(!surface.is_null(), "failed to create wl_surface");
    let xdg_surface = unsafe {
        wl_proxy_marshal_flags(
            xdg_wm_base.obj,
            XDG_WM_BASE_GET_XDG_SURFACE,
            &XDG_SURFACE_INTERFACE as _,
            wl_proxy_get_version(xdg_wm_base.obj),
            0,
            core::ptr::null_mut::<()>(),
            surface,
        )
    };
    assert!(!xdg_surface.is_null(), "failed to create xdg_surface");
    let res = unsafe {
        wl_proxy_add_listener(
            xdg_surface,
            [xdg_surface_handle_configure as *const _].as_mut_ptr(),
            core::ptr::null_mut(),
        )
    };
    assert!(res >= 0, "xdg_surface listener adding error");
    let xdg_toplevel = unsafe {
        wl_proxy_marshal_flags(
            xdg_surface,
            XDG_SURFACE_GET_TOPLEVEL,
            &XDG_TOPLEVEL_INTERFACE,
            wl_proxy_get_version(xdg_surface),
            0,
            core::ptr::null_mut::<()>(),
        )
    };
    assert!(!xdg_toplevel.is_null(), "failed to create xdg_toplevel");
    let res = unsafe {
        wl_proxy_add_listener(
            xdg_toplevel,
            [
                xdg_toplevel_handle_configure as *const _,
                xdg_toplevel_handle_close as *const _,
                xdg_toplevel_handle_configure_bounds as *const _,
                xdg_toplevel_handle_wm_capabilities as *const _,
            ]
            .as_mut_ptr(),
            core::ptr::null_mut(),
        )
    };
    assert!(res >= 0, "xdg_toplevel listener adding error");
    unsafe {
        wl_proxy_marshal_flags(
            xdg_toplevel,
            XDG_TOPLEVEL_SET_APP_ID,
            core::ptr::null(),
            wl_proxy_get_version(xdg_toplevel),
            0,
            c"io.ct2.bedrock.examples.wayland".as_ptr(),
        );
    }
    unsafe {
        wl_proxy_marshal_flags(
            xdg_toplevel,
            XDG_TOPLEVEL_SET_TITLE,
            core::ptr::null(),
            wl_proxy_get_version(xdg_toplevel),
            0,
            c"Bedrock Examples (Wayland Native)".as_ptr(),
        );
    }

    unsafe {
        wl_proxy_marshal_flags(
            surface,
            WL_SURFACE_COMMIT,
            core::ptr::null(),
            wl_proxy_get_version(surface),
            0,
        );
    }

    let mut vk_instance = br::InstanceBuilder::new("Bedrock Examples Wayland Native", (0, 1, 0), "", (0, 1, 0));
    vk_instance
        .add_layer("VK_LAYER_KHRONOS_validation")
        .add_extensions(["VK_KHR_surface", "VK_KHR_wayland_surface", "VK_EXT_debug_utils"])
        .set_api_version(1, 3, 0);
    let vk_instance = Rc::new(vk_instance.create().unwrap());
    let _vk_debugger = br::DebugUtilsMessengerCreateInfo::new(vk_debug_msg)
        .filter_type(br::DebugUtilsMessageTypeFlags::VALIDATION.and_performance())
        .filter_severity(br::DebugUtilsMessageSeverityFlags::ERROR.and_warning())
        .create(vk_instance.clone())
        .unwrap();
    let vk_pdev = vk_instance.iter_physical_devices().unwrap().next().unwrap();
    let vk_surface = (&vk_pdev).new_surface_wayland(con as _, surface as _).unwrap();

    let vk_graphics_queue_family_index = vk_pdev
        .queue_family_properties()
        .find_matching_index(br::QueueFlags::GRAPHICS)
        .unwrap();
    let mut vk_device = br::DeviceBuilder::new(&vk_pdev);
    vk_device
        .add_extensions(["VK_KHR_swapchain"])
        .add_queue(br::DeviceQueueCreateInfo::new(vk_graphics_queue_family_index).add(0.0))
        .add_extra_features(br::vk::VkPhysicalDeviceSynchronization2Features {
            sType: br::vk::VkPhysicalDeviceSynchronization2Features::TYPE,
            pNext: core::ptr::null_mut(),
            synchronization2: true as _,
        });
    let vk_device = Rc::new(vk_device.create().unwrap());
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
    .pre_transform(surface_props.current_transform())
    .composite_alpha(
        if surface_props
            .supported_composite_alpha()
            .has(br::CompositeAlphaFlags::OPAQUE)
        {
            br::CompositeAlpha::Opaque
        } else {
            br::CompositeAlpha::Inherit
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

    let renderpass = br::RenderPassBuilder2::new(
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
    )
    .create(vk_device.clone())
    .unwrap();

    let framebuffers = backbuffers
        .iter()
        .map(|bb| {
            br::FramebufferBuilder::new_with_attachment(
                &renderpass,
                bb.subresource_range(br::AspectMask::COLOR, 0..1, 0..1)
                    .view_builder()
                    .create()?,
            )
            .create()
        })
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut cp = br::CommandPoolBuilder::new(vk_graphics_queue_family_index)
        .create(vk_device.clone())
        .unwrap();
    let mut cb = cp.alloc(framebuffers.len() as _, true).unwrap();
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
                &br::SubpassBeginInfo::new(br::vk::VK_SUBPASS_CONTENTS_INLINE),
            )
            .end_render_pass_2(&br::SubpassEndInfo::new())
            .end()
            .unwrap();
    }

    let mut fence = br::FenceBuilder::new().create(vk_device.clone()).unwrap();
    let bb_index = vk_swapchain
        .acquire_next(
            None,
            br::CompletionHandler::<_, &br::SemaphoreObject<Rc<br::DeviceObject<&br::InstanceObject>>>>::Host(&fence),
        )
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
            Some(&mut fence),
        )
        .unwrap();
    fence.wait().unwrap();
    fence.reset().unwrap();
    vk_queue
        .present(br::PresentInfo::new(
            &[] as &[br::SemaphoreRef<br::SemaphoreObject<Rc<br::DeviceObject<&br::InstanceObject>>>>],
            &[vk_swapchain.as_transparent_ref()],
            &[bb_index],
        ))
        .unwrap();
    vk_queue.wait().unwrap();

    while unsafe { wl_display_dispatch(con) != -1 } {}

    drop(framebuffers);
    drop(renderpass);
    drop(backbuffers);
    drop(vk_swapchain);

    unsafe {
        wl_display_disconnect(con);
    }
}

extern "system" fn vk_debug_msg(
    messageSeverity: br::vk::VkDebugUtilsMessageSeverityFlagBitsEXT,
    messageTypes: br::vk::VkDebugUtilsMessageTypeFlagsEXT,
    pCallbackData: *const br::vk::VkDebugUtilsMessengerCallbackDataEXT,
    pUserData: *mut c_void,
) -> br::vk::VkBool32 {
    let msg = unsafe { core::ffi::CStr::from_ptr((*pCallbackData).pMessage).to_str().unwrap() };

    eprintln!("*vk_debug_msg* {msg}");

    true as _
}

extern "C" fn xdg_surface_handle_configure(data: *mut c_void, xdg_surface: *mut wl_proxy, serial: c_uint) {
    println!("surface configure: {serial}");

    unsafe {
        wl_proxy_marshal_flags(
            xdg_surface,
            XDG_SURFACE_ACK_CONFIGURE,
            core::ptr::null(),
            wl_proxy_get_version(xdg_surface),
            0,
            serial,
        )
    };
}

extern "C" fn xdg_toplevel_handle_configure(
    data: *mut c_void,
    xdg_toplevel: *mut wl_proxy,
    width: c_int,
    height: c_int,
    states: *const wl_array,
) {
    let states = unsafe { (*states).as_slice_unchecked::<c_int>() };

    println!("toplevel configure: {width} {height} {states:?}");
}

extern "C" fn xdg_toplevel_handle_close(data: *mut c_void, xdg_toplevel: *mut wl_proxy) {
    println!("toplevel close");
}

extern "C" fn xdg_toplevel_handle_configure_bounds(
    data: *mut c_void,
    xdg_toplevel: *mut wl_proxy,
    width: c_int,
    height: c_int,
) {
    println!("toplevel configure_bounds: {width} {height}");
}

extern "C" fn xdg_toplevel_handle_wm_capabilities(
    data: *mut c_void,
    xdg_toplevel: *mut wl_proxy,
    capabilities: *const wl_array,
) {
    let caps = unsafe { (*capabilities).as_slice_unchecked::<c_int>() };

    println!("toplevel wm_capabilities: {caps:?}");
}

extern "C" fn registry_handle_global(
    data: *mut c_void,
    registry: *mut wl_proxy,
    name: u32,
    interface: *const c_char,
    version: u32,
) {
    let interfaces = unsafe { &mut *(data as *mut WaylandInterfaces) };
    let interface_name = unsafe { core::ffi::CStr::from_ptr(interface).to_str().unwrap() };

    println!("[wayland global registry] {interface_name} version: {version}, name: {name}");

    if interface_name == "wl_compositor" {
        match interfaces.compositor {
            None => {
                let id = unsafe {
                    wl_proxy_marshal_flags(
                        registry,
                        WL_REGISTRY_BIND,
                        core::ptr::addr_of!(wl_compositor_interface),
                        version,
                        0,
                        name,
                        interface,
                        version,
                        core::ptr::null_mut::<()>(),
                    )
                };
                assert!(!id.is_null(), "failed to bind wl_compositor");
                interfaces.compositor = Some(WaylandGlobalInterface { obj: id, version });
            }
            Some(ref v) if v.version > version => {
                // upgrade
                eprintln!("compositor upgrade found: {} -> {version}", v.version);
            }
            Some(_) => (),
        }
    }

    if interface_name == "xdg_wm_base" {
        match interfaces.xdg_wm_base {
            None => {
                let id = unsafe {
                    wl_proxy_marshal_flags(
                        registry,
                        WL_REGISTRY_BIND,
                        &XDG_WM_BASE_INTERFACE as *const _,
                        version,
                        0,
                        name,
                        interface,
                        version,
                        core::ptr::null_mut::<()>(),
                    )
                };
                assert!(!id.is_null(), "failed to bind xdg_wm_base");
                interfaces.xdg_wm_base = Some(WaylandGlobalInterface { obj: id, version });
            }
            Some(ref v) if v.version > version => {
                // upgrade
                eprintln!("xdg_wm_base upgrade found: {} -> {version}", v.version);
            }
            Some(_) => (),
        }
    }
}

extern "C" fn registry_handle_global_remove(data: *mut c_void, registry: *mut wl_proxy, name: u32) {
    println!("[wayland global registry remove] name: {name}");
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

pub const WL_DISPLAY_GET_REGISTRY: u32 = 1;
pub const WL_REGISTRY_BIND: u32 = 0;
pub const WL_COMPOSITOR_CREATE_SURFACE: u32 = 0;
pub const WL_SURFACE_COMMIT: u32 = 6;
pub const XDG_WM_BASE_GET_XDG_SURFACE: u32 = 2;
pub const XDG_SURFACE_GET_TOPLEVEL: u32 = 1;
pub const XDG_SURFACE_ACK_CONFIGURE: u32 = 4;
pub const XDG_TOPLEVEL_SET_TITLE: u32 = 2;
pub const XDG_TOPLEVEL_SET_APP_ID: u32 = 3;

#[link(name = "wayland-client")]
extern "C" {
    pub static wl_seat_interface: wl_interface;
    pub static wl_registry_interface: wl_interface;
    pub static wl_compositor_interface: wl_interface;
    pub static wl_surface_interface: wl_interface;
    pub static wl_output_interface: wl_interface;

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
    pub fn wl_proxy_add_listener(proxy: *mut wl_proxy, implementation: *mut *const c_void, data: *mut c_void) -> c_int;
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
