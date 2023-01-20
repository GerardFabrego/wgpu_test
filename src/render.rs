use std::borrow::Cow;

use crate::cube::*;
use crate::init_wgpu;
use crate::transforms;
use crate::vertex::Vertex;
use bytemuck::cast_slice;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use winit::{event::WindowEvent, window::Window};

pub struct Render {
    pub init: init_wgpu::InitWgpu,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
}

fn vertex(position: [i8; 3], color: [i8; 3]) -> Vertex {
    Vertex {
        position: [
            position[0] as f32,
            position[1] as f32,
            position[2] as f32,
            1.0,
        ],
        color: [color[0] as f32, color[1] as f32, color[2] as f32, 1.0],
    }
}

fn create_vertices() -> (Vec<Vertex>, Vec<u16>) {
    let (position, color, indices) = cube_data();

    let mut data: Vec<Vertex> = Vec::with_capacity(position.len());

    for i in 0..position.len() {
        data.push(vertex(position[i], color[i]));
    }
    (data, indices)
}

const IS_PERSPECTIVE: bool = true;

impl Render {
    pub async fn new(window: &Window) -> Self {
        let init = init_wgpu::InitWgpu::init_wgpu(window).await;

        // Create buffers

        let (vertex_data, index_data) = create_vertices();

        let vertex_buffer = init.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: cast_slice(&vertex_data), // bytemuck::cast_slice()
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = init.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: cast_slice(&index_data),
            usage: wgpu::BufferUsages::INDEX,
        });

        // Load the shaders from disk

        let shader = init
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
            });

        // Create uniform buffer

        let camera_position = (3.0, 1.5, 3.0).into();
        let look_direction = (0.0, 0.0, 0.0).into();
        let up_direction = cgmath::Vector3::unit_y();
        let model_mat =
            transforms::create_transforms([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
        let (_, _, view_project_mat) = transforms::create_view_projection(
            camera_position,
            look_direction,
            up_direction,
            init.config.width as f32 / init.config.height as f32,
            IS_PERSPECTIVE,
        );
        let mvp_mat = view_project_mat * model_mat;
        let mvp_ref: &[f32; 16] = mvp_mat.as_ref();

        let uniform_buffer = init.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: cast_slice(mvp_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group_layout =
            init.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: Some("Uniform Bind Group Layout"),
                });

        let uniform_bind_group = init.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform bind group"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // Create pipeline

        let pipeline_layout = init
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Pipeline layout"),
                bind_group_layouts: &[&uniform_bind_group_layout],
                push_constant_ranges: &[],
            });

        let pipeline = init
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[Vertex::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: init.config.format,
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent::REPLACE,
                            alpha: wgpu::BlendComponent::REPLACE,
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleStrip,
                    strip_index_format: None,
                    ..Default::default()
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth24Plus,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::LessEqual,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            });

        Self {
            init,
            pipeline,
            vertex_buffer,
            index_buffer,
            uniform_bind_group,
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let frame = self.init.surface.get_current_texture().unwrap();

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let depth_texture = self.init.device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: self.init.config.width,
                height: self.init.config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth24Plus,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
        });
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .init
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.2,
                            g: 0.247,
                            b: 0.314,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &&depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: false,
                    }),
                    stencil_ops: None,
                }),
            });
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);

            render_pass.draw_indexed(0..36, 0, 0..1);
        }

        self.init.queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }

    pub fn update(&mut self) {}

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.init.size = new_size;
        self.init.config.width = new_size.width;
        self.init.config.height = new_size.height;
        self.init
            .surface
            .configure(&self.init.device, &self.init.config);
    }

    pub fn input(&self, event: &WindowEvent) -> bool {
        false
    }
}
