use imgui_winit_support::{WinitPlatform, winit};

use crate::app_gui::{self};
use glutin::{event_loop::EventLoop, WindowedContext, event};
use imgui_winit_support;
use glutin;
use std::{time::Instant, sync::{Arc, Mutex}};
use log::*;

pub(crate) const TITLE: &str = "Hello, imgui-rs!";
pub(crate) type Window = WindowedContext<glutin::PossiblyCurrent>;
pub(crate) fn start_gui(mut appgui:  Box<app_gui::AppGui>) {
    trace!("Starting ImGui main loop.");
    let (event_loop, window) = create_window();
    let (mut winit_platform, mut imgui_context) = imgui_init(&window);

    // OpenGL context from glow
    let gl = glow_context(&window);

    // OpenGL renderer from this crate
    let mut ig_renderer = imgui_glow_renderer::AutoRenderer::initialize(gl, &mut imgui_context)
        .expect("failed to create renderer");

    let mut last_frame = Instant::now();
    let proxy = event_loop.create_proxy();
    let rc_proxy = Arc::new(Mutex::new(proxy));

    // Standard winit event loop
    event_loop.run(move |event, event_loop_proxy, control_flow|  {
        *control_flow = winit::event_loop::ControlFlow::Wait;
        trace!("Event loop received event : {:?}", event);
        match event {
        glutin::event::Event::NewEvents(_) => {
            let now = Instant::now();
            imgui_context
                .io_mut()
                .update_delta_time(now.duration_since(last_frame));
            last_frame = now;
        }
        glutin::event::Event::MainEventsCleared => {
            winit_platform
                .prepare_frame(imgui_context.io_mut(), window.window())
                .unwrap();
            window.window().request_redraw();
        }
        glutin::event::Event::RedrawRequested(_) => {
            appgui.render(
                &mut ig_renderer,
                &mut imgui_context,
                &window,
                &mut winit_platform,
                rc_proxy.clone(),
            );
        }
        glutin::event::Event::WindowEvent {
            event: glutin::event::WindowEvent::CloseRequested,
            ..
        } => {
            *control_flow = glutin::event_loop::ControlFlow::Exit;
        }
        event => {
            winit_platform.handle_event(imgui_context.io_mut(), window.window(), &event);
        }
    }
    });
}


pub(crate) fn create_window() -> (EventLoop<()>, Window) {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new()
        .with_title(TITLE)
        .with_inner_size(glutin::dpi::LogicalSize::new(1024, 768));
    let window = glutin::ContextBuilder::new()
        .with_vsync(true)
        .build_windowed(window, &event_loop)
        .expect("could not create window");
    let window = unsafe {
        window
            .make_current()
            .expect("could not make window context current")
    };
    (event_loop, window)
}

pub(crate) fn glow_context(window: &Window) -> glow::Context {
    unsafe { glow::Context::from_loader_function(|s| window.get_proc_address(s).cast()) }
}

pub(crate) fn imgui_init(window: &Window) -> (WinitPlatform, imgui::Context) {
    let mut imgui_context = imgui::Context::create();
    imgui_context.set_ini_filename(None);

    let mut winit_platform = WinitPlatform::init(&mut imgui_context);
    winit_platform.attach_window(
        imgui_context.io_mut(),
        window.window(),
        imgui_winit_support::HiDpiMode::Rounded,
    );

    imgui_context
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    imgui_context.io_mut().font_global_scale = (1.0 / winit_platform.hidpi_factor()) as f32;

    (winit_platform, imgui_context)
}
