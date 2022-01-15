use super::log_test;

use std::iter;
use std::time::Instant;

use chrono::Timelike;
use egui::FontDefinitions;
use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::{Platform, PlatformDescriptor};
use epi::*;
use winit::event::Event::*;
use winit::event_loop::ControlFlow;

const INITIAL_WIDTH: u32 = 1920;
const INITIAL_HEIGHT: u32 = 1080;


enum Event {
    RequestRedraw,
}

// Need to be translated into one dedicated to NI Framework.
struct NI3DPrototypeRepaintSignal(std::sync::Mutex<winit::event_loop::EventLoopProxy<Event>>);

impl epi::backend::RepaintSignal for NI3DPrototypeRepaintSignal {
    fn request_repaint(&self) {
        self.0.lock().unwrap().send_event(Event::RequestRedraw).ok();
    }
}

pub fn main_egui_example() {
    let event_loop = winit::event_loop::EventLoop::with_user_event();
    let repaint_signal = std::sync::Arc::new(NI3DPrototypeRepaintSignal(std::sync::Mutex::new(
        event_loop.create_proxy()
    )));
    let window = winit::window::WindowBuilder::new()
        .with_decorations(true)
        .with_resizable(true)
        .with_transparent(true)
        .with_title("NI3DPrototype")
        .with_inner_size(winit::dpi::PhysicalSize {
            width: INITIAL_WIDTH,
            height: INITIAL_HEIGHT,
        })
        .build(&event_loop)
        .unwrap();

    let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
    let surface = unsafe { instance.create_surface(&window) };
    let adapter = pollster::block_on( instance.request_adapter( &wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    }))
    .unwrap();
    let (mut device, queue) = pollster::block_on( adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::default(),
            limits: wgpu::Limits::default(),
            label: None,
        },
        None,
    ))
    .unwrap();
    let size = window.inner_size();
    let surface_format = surface.get_preferred_format(&adapter).unwrap();
    let mut surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width as u32,
        height: size.height as u32,
        present_mode: wgpu::PresentMode::Mailbox,
    };
    surface.configure(&device, &surface_config);

    let mut platform = Platform::new(PlatformDescriptor {
        physical_width: size.width,
        physical_height: size.height,
        scale_factor: window.scale_factor(),
        font_definitions: FontDefinitions::default(),
        style: Default::default(),
    });

    let mut render_pass = RenderPass::new(&device, surface_format, 1);
    let mut app = egui_demo_lib::WrapApp::default();

    let start_time = Instant::now();
    let mut previous_frame_time = None;
    log_test::start_log();
    log_test::inner_log_test::print_test();

    event_loop.run(move | event, _, control_flow |{
        platform.handle_event(&event);

        match event {
            RedrawRequested(..) => {
                platform.update_time(start_time.elapsed().as_secs_f64());
                let output_frame = match surface.get_current_texture() {
                    Ok(frame) => frame,
                    Err(e) => {
                        eprintln!("Dropped frame with error: {}", e);
                        return
                    }
                };
                let output_view = output_frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                let render_start = Instant::now();
                platform.begin_frame();
                let mut app_output = epi::backend::AppOutput::default();
                let frame_data = epi::backend::FrameData {
                    info: epi::IntegrationInfo {
                        name: "",
                        web_info: None,
                        cpu_usage: previous_frame_time,
                        // seconds_since_midnight: Some(seconds_since_midnight()),
                        native_pixels_per_point: Some(window.scale_factor() as _),
                        prefer_dark_mode: None,
                    },
                    output: app_output,
                    repaint_signal: repaint_signal.clone(),
                };
                let mut frame = epi::Frame::new(frame_data);
                app.update(&platform.context(), &mut frame);
                let (_output, paint_commands) = platform.end_frame(Some(&window));
                let paint_jobs = platform.context().tessellate(paint_commands);

                let frame_time = (Instant::now() - start_time).as_secs_f64() as f32;
                previous_frame_time = Some(frame_time);

                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor{
                    label: Some("encoder"),
                });

                let screen_descriptor = ScreenDescriptor {
                    physical_width: surface_config.width,
                    physical_height: surface_config.height,
                    scale_factor: window.scale_factor() as f32,
                };

                render_pass.update_texture(&device, &queue, &platform.context().texture());
                render_pass.update_user_textures(&device, &queue);
                render_pass.update_buffers(&device, &queue, &paint_jobs, &screen_descriptor);
                render_pass.execute(
                    &mut encoder,
                    &output_view,
                    &paint_jobs, 
                    &screen_descriptor,
                    Some(wgpu::Color::BLACK),
                )
                .unwrap();
                queue.submit(iter::once(encoder.finish()));

                output_frame.present();
            }
            MainEventsCleared | UserEvent(Event::RequestRedraw) => {
                window.request_redraw();
            }
            WindowEvent {event, .. } => match event {
                winit::event::WindowEvent::Resized(size) => {
                    surface_config.width = size.width;
                    surface_config.height = size.height;
                    surface.configure(&device, &surface_config);
                }
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            }
            _ => {}
        }
    })

}

#[inline]
pub fn seconds_since_midnight() -> f64 {
    let time = chrono::Local::now().time();
    time.num_seconds_from_midnight() as f64 + 1e-9 * (time.nanosecond() as f64)
}