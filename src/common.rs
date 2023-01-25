use crate::light::Light;
use crate::render::Render;
use crate::vertex::Vertex;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

pub fn run(mesh_data: &Vec<Vertex>, light: Light) {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    window.set_title("Hello wgpu!");
    env_logger::init();

    let mut render = pollster::block_on(Render::new(&window, mesh_data, light));

    let render_start_time = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::MainEventsCleared => window.request_redraw(),
            Event::DeviceEvent { ref event, .. } => {
                render.input(event);
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                // if !render.input(event) {
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
                        render.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        render.resize(**new_inner_size);
                    }
                    _ => {}
                };
                // }
            }
            Event::RedrawRequested(_) => {
                let now = std::time::Instant::now();
                let dt = now - render_start_time;

                render.update(dt);
                match render.render() {
                    Ok(_) => {}
                    Err(e) => eprintln!("{:?}", e),
                }
            }

            _ => {}
        }
    });
}
