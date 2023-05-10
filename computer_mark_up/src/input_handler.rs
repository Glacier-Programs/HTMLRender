use winit::event::{WindowEvent, Event};

pub struct InputHandler{
    mouse_position: [f32; 2],
    mouse_buttons: [bool; 3]
}

impl InputHandler{
    pub fn new_default() -> Self{
        Self{
            mouse_position: [0.0, 0.0],
            mouse_buttons: [false, false, false]
        }
    }

    pub fn handle_mouse_motion(&mut self, new_pos: &winit::dpi::PhysicalPosition<f64>){
        self.mouse_position = [new_pos.x as f32, new_pos.y as f32]
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) -> bool{
        match event{
            _ => false
        }
    }

    pub fn get_mouse_pos(&self) -> [f32; 2]{
        self.mouse_position
    }
}