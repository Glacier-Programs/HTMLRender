// this corresponds to ComponentInput in 'component_shader.wgsl'
#[repr(C)]
#[derive(Copy, Clone, Debug, )]
pub struct ComponentVertex{
    pub position: [f32; 2]
}

impl ComponentVertex{
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ComponentVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2
                },
            ]
        }
    }
}