use crate::render::vertex::ComponentVertex;
use crate::input_handler::InputHandler;
use crate::collision;

use super::ComponentObject;

// a component whose texture changes when it is hovered
pub struct HoverComponent{
    top_left_corner: [f32; 2],
    width: f32,
    height: f32,
    base_texture: u32,
    hover_texture: u32,
    current_texture: u32
}

impl HoverComponent{
    pub fn new(tlc: [f32; 2], width: f32, height: f32, base: u32, hover: u32) -> Self{
        Self { 
            top_left_corner: tlc, 
            width, 
            height, 
            base_texture: base, 
            hover_texture: hover,
            current_texture: 0u32
        }
    }
}

impl ComponentObject for HoverComponent{
    fn update(&mut self, input: &InputHandler) -> crate::update_commands::UpdateCommand {
        // do the collision detection
        let mcoords = input.get_mouse_pos();
        let collided = collision::point_on_rect(
            self.top_left_corner, 
            self.width, 
            self.height, 
            mcoords
        );
        self.current_texture = self.base_texture * collided as u32 + self.hover_texture * !collided as u32;
        crate::update_commands::UpdateCommand::Void
    }

    fn get_vertices(&self) -> [crate::render::vertex::ComponentVertex; 4] {
        let trc = [self.top_left_corner[0] + self.width, self.top_left_corner[1]              ];
        let blc = [self.top_left_corner[0],              self.top_left_corner[1] + self.height];
        let brc = [self.top_left_corner[0] + self.width, self.top_left_corner[1] + self.height];
        [
            ComponentVertex{ position: self.top_left_corner, texture_index: self.current_texture, texture_coords: [0.0, 1.0] },
            ComponentVertex{ position: trc,                  texture_index: self.current_texture, texture_coords: [1.0, 0.0] },
            ComponentVertex{ position: blc,                  texture_index: self.current_texture, texture_coords: [0.0, 0.0] },
            ComponentVertex{ position: brc,                  texture_index: self.current_texture, texture_coords: [1.0, 0.0] }
        ]
    }
}