#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
pub struct ScreenDetails{
    width: u32,
    height: u32,
    scroll: [f32; 2]
}

impl ScreenDetails{
    pub fn new(config: &wgpu::SurfaceConfiguration) -> Self{
        Self { 
            width: config.width, 
            height: config.height, 
            scroll: [0.0, 0.0]
        }
    }
}