use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod render;

mod component;
mod update_commands;
mod input_handler;

mod file_reader;

/*
 * There is a WindowState and a SceneState
 * Window controls window events and rendering
 * Scene controls everything else including:
 * - Nonwindow Input
 * - Component Management
 */

async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut ws = render::window_state::WindowState::new(window).await;
    let mut ih = input_handler::InputHandler::new_default();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == ws.window().id() => match event {
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
                ws.resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                // new_inner_size is &&mut so we have to dereference it twice
                ws.resize(**new_inner_size);
            }
            _ => {}
        },
        Event::MainEventsCleared => ws.window().request_redraw(),
        Event::RedrawRequested(id) if id == ws.window().id() => {
            match ws.render(Vec::new()){
                Ok(_) => {},
                Err(wgpu::SurfaceError::Lost) => ws.resize(*ws.size()),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }
        _ => {}
    });
}

fn main(){
    pollster::block_on(run());
}
