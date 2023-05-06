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
}