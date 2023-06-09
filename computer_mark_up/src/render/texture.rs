use image::GenericImageView;

use super::color::Color;

#[derive(Debug)]
pub enum TextureType{
    // The same color across a texture
    Color(Color),
    Texture(Texture)
}

#[derive(Debug)]
pub struct Texture{
    pub(crate) label: String,
    pub(crate) texture_surface: wgpu::Texture,
    pub(crate) sampler: wgpu::Sampler,
    pub(crate) view: wgpu::TextureView,
}

impl Texture{
    pub fn new(img_data: &[u8], label: &str, device: &wgpu::Device, queue: &wgpu::Queue, texture_bind_group_layout: wgpu::BindGroupLayout) -> Self{
        let diffuse_image = image::load_from_memory(img_data).unwrap();
        let rgba_data = diffuse_image.to_rgba8();
        
        let dimensions = diffuse_image.dimensions();

        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1 // technically 3d, but only need 1
        };
        let texture_surface = device.create_texture(
            &wgpu::TextureDescriptor {
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                // Most images are stored using sRGB so we need to reflect that here.
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                // TEXTURE_BINDING tells wgpu that we want to use this texture in shaders
                // COPY_DST means that we want to copy data to this texture
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: Some(label),
                view_formats: &[],
            }
        );
        
        // load rgb data onto the texture surface
        queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::ImageCopyTexture {
                texture: &texture_surface,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // The actual pixel data
            &rgba_data,
            // The layout of the texture
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0),
                rows_per_image: std::num::NonZeroU32::new(dimensions.1),
            },
            texture_size,
        );
        
        let view = texture_surface.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self{
            label: String::from(label),
            texture_surface,
            sampler,
            view
        }
    }

}