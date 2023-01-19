mod render;
mod vertex;

use render::Render;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

fn main() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    window.set_title("Hello wgpu!");
    env_logger::init();

    let mut render = pollster::block_on(Render::new(&window));

    event_loop.run(move |event, _, control_flow| {
        // let _ = (&instance, &adapter, &shader, &pipeline_layout);
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                // Recreate the surface with the new size
                render.config.width = size.width;
                render.config.height = size.height;
                render.surface.configure(&render.device, &render.config);
            }
            Event::RedrawRequested(_) => {
                render.render();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}
