use std::mem::transmute;

use super::texture::Texture;

// rgba
// Where each value is between 0 and 1
// where 0 is 0 and 1 is 255
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Color([f32; 4]);

impl Color{
    pub fn new(rgba: [f32; 4]) -> Self{
        Self(rgba)
    }

    pub fn as_bytes(&self) -> Box<[u8]>{
        // This method turns Color into just being an array of unsized 8 bit
        // integers. Since these are basically just bytes, it's treated as a byte
        // The length is 16 since that's the size of the Color struct
        // Unsafe is used since transmute is an unsafe function
        unsafe{
            Box::new(
                transmute::<Color, [u8; 16]>(*self)
            )
        }
    }

    pub fn match_buffer(&self) -> Box<[u8]>{
        // for some reason the data doesn't align with the texture buffer,
        // so padding needs to be added at the start
        let bytes = self.as_bytes();
        let padding = Box::new([0u8, 0u8]);
        let combined = Vec::from([bytes, padding]).concat();
        Box::from( combined.as_slice() )
    }

    pub fn as_texture(&self, device: &wgpu::Device, queue: &wgpu::Queue, bind_group_layout: &wgpu::BindGroupLayout) -> Texture{
        /*
         * Create a one pixel image with the color being equal to that of self
         */
        let rgba = self.0;
        let mut pixel_image = image::RgbaImage::new(1, 1);
        pixel_image.put_pixel(
            0, 
            0, 
            image::Rgba(rgba.map(|x| ( x * 255.0 ) as u8))
        );

        let texture_size = wgpu::Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        };
        let texture_surface = device.create_texture(
            &wgpu::TextureDescriptor {
                // All textures are stored as 3D, we represent our 2D texture
                // by setting depth to 1.
                size: texture_size,
                mip_level_count: 1, // We'll talk about this a little later
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                // Most images are stored using sRGB so we need to reflect that here.
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                // TEXTURE_BINDING tells wgpu that we want to use this texture in shaders
                // COPY_DST means that we want to copy data to this texture
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                // format!() returns a String, so [..] is used to get a str, which is then borrowed
                // as label requires a &str
                label: Some(&format!("color: {:?}", self)[..]),
                // This is the same as with the SurfaceConfig. It
                // specifies what texture formats can be used to
                // create TextureViews for this texture. The base
                // texture format (Rgba8UnormSrgb in this case) is
                // always supported. Note that using a different
                // texture format is not supported on the WebGL2
                // backend.
                view_formats: &[],
            }
        );

        // This tells the gpu to draw onto the texture surface
        // If this isn't called then the texture will just be blank
        // or random noise
        queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::ImageCopyTexture {
                texture: &texture_surface,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // The actual pixel data
            // dereferenced to drop the Box<> then borrowed since 
            // [u8] is unsized
            &pixel_image,
            // The layout of the texture
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4),
                rows_per_image: std::num::NonZeroU32::new(1)
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

        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
            );

        Texture { 
            label: format!("Color: {:?}", self), 
            texture_surface, 
            sampler, 
            view, 
            bind_group
        }
     }
}