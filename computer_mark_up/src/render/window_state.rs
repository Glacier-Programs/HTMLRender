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
    default_render_pipeline: wgpu::RenderPipeline,
    screen_details: ScreenDetails,
    screen_details_bind_group_layout: wgpu::BindGroupLayout,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    textures: Vec<Texture>
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
                features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
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
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

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
            default_render_pipeline: render_pipeline,
            screen_details,
            screen_details_bind_group_layout,
            texture_bind_group_layout,
            textures: Vec::new()
        }
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
        let color_as_text = color.as_texture(&self.device, &self.queue, &self.texture_bind_group_layout);
        self.textures.push(color_as_text);
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
        let indices: Vec<u32> = empty_indices
            .iter()
            .enumerate()
            .map(|(index, _value)| {
                // _value is discarded as it is random noise
                let index_in_order_array = index % 6;
                let unedited_value = QUAD_VERTEX_ORDER[index_in_order_array];
                let increase = index as u32 / 6;
                unedited_value + increase
            })
            .collect();
        
        let vertices = components
            .iter()
            .map(|component| component.get_vertices() )
            .collect::<Vec<[ComponentVertex; 4]>>()
            .concat();

        let quad_index_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Quad Index Buffer"),
                contents: bytemuck::cast_slice(indices.as_slice()),
                usage: wgpu::BufferUsages::INDEX
            }
        );

        let quad_vertex_buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Quad Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices.as_slice()),
                usage: wgpu::BufferUsages::VERTEX
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
            render_pass.set_bind_group(1, &self.textures[0].bind_group, &[]);

            render_pass.set_vertex_buffer(0, quad_vertex_buffer.slice(..));
            render_pass.set_index_buffer(quad_index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..components.len() as u32*6, 0, 0..1);
        }
    
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())
    
    }
}