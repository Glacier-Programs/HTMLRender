use crate::render::{vertex::ComponentVertex, texture::Texture, color::Color};
use super::{ComponentObject, DefaultBuild, CustomBuildParameters};

pub struct SquareComponentParameters{
    pub top_left_corner: [f32; 2],
    pub width: f32,
    pub height: f32
}
impl CustomBuildParameters for SquareComponentParameters{}

/*
 * This is just a square rendered onto the screen
 */

pub struct SquareComponent{
    top_left_corner: [f32; 2],
    width: f32,
    height: f32,
    texture: Texture
}

impl SquareComponent{
    pub fn new(corner: [f32; 2], width: f32, height: f32, texture: Texture) -> Self{
        Self { 
            top_left_corner: corner, 
            width, 
            height,
            texture
        }
    }
}

impl ComponentObject for SquareComponent{
    fn get_vertices(&self) -> [ComponentVertex; 4] {
        let trc = [self.top_left_corner[0] + self.width, self.top_left_corner[1]              ];
        let blc = [self.top_left_corner[0],              self.top_left_corner[1] + self.height];
        let brc = [self.top_left_corner[0] + self.width, self.top_left_corner[1] + self.height];
        [
            ComponentVertex{ position: self.top_left_corner, texture_index: 0, texture_coords: [0.0, 1.0] },
            ComponentVertex{ position: trc,                  texture_index: 0, texture_coords: [1.0, 0.0] },
            ComponentVertex{ position: blc,                  texture_index: 0, texture_coords: [0.0, 0.0] },
            ComponentVertex{ position: brc,                  texture_index: 0, texture_coords: [1.0, 0.0] }
        ]
    }

    fn render(&self) -> &crate::render::texture::Texture {
        &self.texture
    }
}

impl DefaultBuild for SquareComponent{
    fn build_default(device: &wgpu::Device, queue: &wgpu::Queue, config: &wgpu::SurfaceConfiguration, texture_bind_group_layout: &wgpu::BindGroupLayout) -> Self {
        let color = Color::new([1.0, 0.0, 0.0, 1.0]);
        let texture = color.as_texture(device, queue, texture_bind_group_layout);
        Self { 
            top_left_corner: [0.0,0.0], 
            width: 400.0, 
            height: 300.0, 
            texture 
        }
    }
}