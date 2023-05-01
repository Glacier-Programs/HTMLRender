use crate::render::vertex::ComponentVertex;
use super::ComponentObject;

/*
 * This is just a square rendered onto the screen
 */

pub struct SquareComponent{
    top_left_corner: [f32; 2],
    width: f32,
    height: f32
}

impl SquareComponent{
    pub fn new(corner: [f32; 2], width: f32, height: f32) -> Self{
        Self { 
            top_left_corner: corner, 
            width, 
            height 
        }
    }
}

impl ComponentObject for SquareComponent{
    fn get_vertices(&self) -> [ComponentVertex; 4] {
        let trc = [self.top_left_corner[0] + self.width, self.top_left_corner[1]              ];
        let blc = [self.top_left_corner[0],              self.top_left_corner[1] - self.height];
        let brc = [self.top_left_corner[0] + self.width, self.top_left_corner[1] - self.height];
        [
            ComponentVertex{ position: self.top_left_corner },
            ComponentVertex{ position: trc },
            ComponentVertex{ position: blc },
            ComponentVertex{ position: brc }
        ]
    }
}