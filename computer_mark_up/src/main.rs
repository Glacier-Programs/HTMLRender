use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod render;
mod collision;

mod components;
mod update_commands;
mod input_handler;
mod scene_state;

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
    let mut ss = scene_state::SceneState::new_empty();

    // Create textures
    ws.load_color(render::color::Color::new([1.0, 0.5, 1.0, 1.0]));
    ws.load_color(render::color::Color::new([0.0, 1.0, 1.0, 1.0]));

    // Create components
    ss.add_component(
        components::SquareComponent::new([0.0, 0.0], 800.0, 600.0)
    );
    ss.add_component(
        components::HoverComponent::new([800.0, 0.0], 800.0, 600.0, 0, 1)
    );

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == ws.window().id() && !ih.handle_window_event(event) => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
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
            ss.update(&ih);
            match ws.render(ss.get_components()){
                Ok(_) => {},
                Err(wgpu::SurfaceError::Lost) => ws.resize(*ws.size()),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        },
        _ => {}
    });
}

fn main(){
    pollster::block_on(run());
}
