use std::num::NonZeroU32; // idk why this is needed, but the official example uses it for texture arrays
use winit::window::Window;
use wgpu::util::DeviceExt;

use crate::components::Component;

use super::{
    vertex::ComponentVertex, 
    screen_details::ScreenDetails,
    texture::Texture, color::Color
};

const QUAD_VERTEX_ORDER: [u32; 6] = [1u32, 2u32, 0u32, 3u32, 2u32, 1u32];


pub struct WindowState {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,
    shader: wgpu::ShaderModule,
    default_render_pipeline: wgpu::RenderPipeline,
    screen_details: ScreenDetails,
    screen_details_bind_group_layout: wgpu::BindGroupLayout,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    textures: Vec<Texture>,
    textures_bind_group: wgpu::BindGroup
}

impl WindowState {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });
        
        // # Safety
        //
        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();
        
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                // This allows arrays in uniforms
                features: wgpu::Features::TEXTURE_BINDING_ARRAY | wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING,
                limits: wgpu::Limits::default(),
                label: None,
            },
            None, // Trace path
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps.formats.iter()
            .copied()
            .filter(|f| f.describe().srgb)
            .next()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        // screen details
        let screen_details = ScreenDetails::new(&config);
        let screen_details_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("screen_details_bind_group_layout"),
        });

        // Shader 
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: NonZeroU32::new(1)
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: NonZeroU32::new(1)
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });
        // bind group requires at least one texture, so just use a placeholder
        let black_texture = Color::new([0.0, 0.0, 0.0, 0.0]).as_texture(&device, &queue);
        let textures_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureViewArray(&[&black_texture.view]),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::SamplerArray(&[&black_texture.sampler]),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
            );

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("component_shader.wgsl").into()),
        });
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Component Render Pipeline Layout"),
            bind_group_layouts: &[
                &screen_details_bind_group_layout,
                &texture_bind_group_layout
            ],
            push_constant_ranges: &[],
        });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Component Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                // What vertex types can be passed in
                buffers: &[
                    ComponentVertex::desc()
                ]
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // standard
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None, // unnecessary
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            shader,
            default_render_pipeline: render_pipeline,
            screen_details,
            screen_details_bind_group_layout,
            texture_bind_group_layout,
            textures: Vec::new(),
            textures_bind_group
        }
    }

    pub fn device(&self) -> &wgpu::Device{
        &self.device
    }

    pub fn config(&self) -> &wgpu::SurfaceConfiguration{
        &self.config
    }

    pub fn queue(&self) -> &wgpu::Queue{
        &self.queue
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn size(&self) -> &winit::dpi::PhysicalSize<u32>{
        &self.size
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn load_color(&mut self, color: Color){
        let color_as_text = color.as_texture(&self.device, &self.queue);
        self.textures.push(color_as_text);
        self.reload_textures()
    }

    pub fn reload_textures(&mut self){
        // This method forces the WindowState to reload each texture in WindowState.textures into the texture bind gorup
        // This needs to match the number of textures present
        self.texture_bind_group_layout = 
            self.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: NonZeroU32::new(self.textures.len() as u32)
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: NonZeroU32::new(self.textures.len() as u32)
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });
        
        // it may be more efficient to make this a for t in self.textures
        // then add every sample and view reference to slices passed into the bind group
        self.textures_bind_group = self.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &self.texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureViewArray(
                            &self.textures.iter()
                            .map(|t| &t.view)
                            .collect::<Vec<&wgpu::TextureView>>()
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::SamplerArray(
                            &self.textures.iter()
                            .map(|t| &t.sampler)
                            .collect::<Vec<&wgpu::Sampler>>()
                        ),
                    }
                ],
                label: Some("textures_bind_group")
            }
        );
        self.reconstruct_pipeline()
    }

    pub fn reconstruct_pipeline(&mut self){
        // Recreates the component render pipeline with all currently loaded textures
        let render_pipeline_layout = self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Component Render Pipeline Layout"),
            bind_group_layouts: &[
                &self.screen_details_bind_group_layout,
                &self.texture_bind_group_layout
            ],
            push_constant_ranges: &[],
        });
        self.default_render_pipeline = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Component Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &self.shader,
                entry_point: "vs_main",
                // What vertex types can be passed in
                buffers: &[
                    ComponentVertex::desc()
                ]
            },
            fragment: Some(wgpu::FragmentState {
                module: &self.shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: self.config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // standard
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None, // unnecessary
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
    }

    pub fn render(&mut self, components: &[Component]) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let screen_details_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Screen Details Buffer"),
                contents: bytemuck::cast_slice(&[self.screen_details]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let screen_details_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.screen_details_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: screen_details_buffer.as_entire_binding(),
                }
            ],
            label: Some("screen_details_bind_group"),
        });

        // create vertex and index buffer
        // to create the vertices, we want to create 
        // >num_rendered< copies of QUAD_VERTEX_ORDER,
        // then we want to go through each copy and increase
        // each value in it by n*6 where n is the 
        // number copy it is
        let num_rendered = components.len();
        let empty_indices: Vec<u32> = vec![0; num_rendered*6usize];

        let mut vertex_buffers: Vec<wgpu::Buffer> = Vec::new();
        for comp in components{
            let vertices = comp.get_vertices();
            let quad_vertex_buffer = self.device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Quad Vertex Buffer"),
                    contents: bytemuck::cast_slice(vertices.as_slice()),
                    usage: wgpu::BufferUsages::VERTEX
                }
            );
            vertex_buffers.push(quad_vertex_buffer);
        }

        let index_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Quad Vertex Buffer"),
                contents: bytemuck::cast_slice(&QUAD_VERTEX_ORDER),
                usage: wgpu::BufferUsages::INDEX
            }
        );

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.default_render_pipeline);
            render_pass.set_bind_group(0, &screen_details_bind_group, &[]);
            render_pass.set_bind_group(1, &self.textures_bind_group, &[]);

            // index buffer is same for all
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            for buffer in &vertex_buffers{
                render_pass.set_vertex_buffer(0, buffer.slice(..));
                render_pass.draw_indexed(0..6, 0, 0..1);
            }
        }
    
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())
    
    }
}