mod cube;
mod init_wgpu;
mod render;
mod transforms;
mod vertex;

use render::Render;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
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
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !render.input(event) {
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
                }
            }
            Event::RedrawRequested(_) => {
                render.update();
                match render.render() {
                    Ok(_) => {}
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => window.request_redraw(),
            _ => {}
        }
    });
}
