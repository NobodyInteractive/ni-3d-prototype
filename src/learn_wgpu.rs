mod the_surface;
use the_surface as impl_mod;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window}
};

use async_trait::async_trait;

#[async_trait]
trait Tutorial {
    async fn new(window: &Window) -> Self;
    
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>);
    
    fn input(&mut self, eevnt: &WindowEvent) -> bool;
    
    fn update(&mut self) ;
    
    fn render(&mut self) -> Result<(), wgpu::SurfaceError>;
}


pub fn tut_main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut state = pollster::block_on(impl_mod::State::new(&window));

    event_loop.run(move | event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            state.update();
            match state.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        },
        Event::WindowEvent {
            ref event, window_id
        } if window_id == window.id() => if !state.input(event) { //update!
            match event { 
                

                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    state.resize(*physical_size);
                }
                WindowEvent::ScaleFactorChanged {new_inner_size, ..} => {
                    state.resize(**new_inner_size);
                }
                _ => {}
            }
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ =>   {}
    });    
}