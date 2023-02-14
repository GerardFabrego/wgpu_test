use std::borrow::Cow;
use std::iter;

use crate::camera::*;
use crate::init_wgpu;
use crate::transforms;
use crate::vertex::Vertex;

use bytemuck::cast_slice;
use cgmath::*;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::light::Light;
use winit::{event::*, window::Window};

pub struct Render {
    init: init_wgpu::InitWgpu,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    vertex_uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,

    num_vertices: u32,
    view_mat: Matrix4<f32>,
    project_mat: Matrix4<f32>,
    // mouse_pressed: bool,
}

const IS_PERSPECTIVE: bool = true;
const ANIMATION_SPEED: f32 = 1.0;

impl Render {
    pub async fn new(window: &Window, mesh_data: &Vec<Vertex>, light_data: Light) -> Self {
        let init = init_wgpu::InitWgpu::init_wgpu(window).await;

        let shader = init
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });

        // uniform data
        let camera_position = (3.0, 1.5, 3.0).into();
        let look_direction = (0.0, 0.0, 0.0).into();
        let up_direction = Vector3::unit_y();

        let view_mat = transforms::create_view(camera_position, look_direction, up_direction);

        let project_mat = transforms::create_projection(
            init.config.width as f32 / init.config.height as f32,
            IS_PERSPECTIVE,
        );


        let vertex_uniform_buffer = init.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Uniform Buffer"),
            size: 192,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // create fragment uniform buffer
        let fragment_uniform_buffer = init.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Fragment Uniform Buffer"),
            size: 48,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let light_position: &[f32; 3] = camera_position.as_ref();
        let eye_position: &[f32; 3] = camera_position.as_ref();
        init.queue.write_buffer(
            &fragment_uniform_buffer,
            0,
            bytemuck::cast_slice(light_position),
        );
        init.queue.write_buffer(
            &fragment_uniform_buffer,
            16,
            bytemuck::cast_slice(eye_position),
        );

        // create light uniform buffer
        let light_uniform_buffer = init.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Light Uniform Buffer"),
            size: 48,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // store light parameters
        init.queue.write_buffer(
            &light_uniform_buffer,
            0,
            bytemuck::cast_slice(&[light_data]),
        );

        // Create buffers
        let vertex_buffer = init.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: cast_slice(mesh_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let uniform_bind_group_layout =
            init.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                    label: Some("Uniform Bind Group Layout"),
                });

        let uniform_bind_group = init.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform bind group"),
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: vertex_uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: fragment_uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: light_uniform_buffer.as_entire_binding(),
                },
            ],
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
                    topology: wgpu::PrimitiveTopology::TriangleList,
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
            vertex_uniform_buffer,
            uniform_bind_group,
            num_vertices: mesh_data.len() as u32,
            view_mat,
            project_mat,
            // mouse_pressed: false,
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

        let mut encoder =
            self.init
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

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
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: false,
                    }),
                    stencil_ops: None,
                }),
            });
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.draw(0..self.num_vertices, 0..1);
        }

        self.init.queue.submit(iter::once(encoder.finish()));
        frame.present();

        Ok(())
    }
    pub fn update(&mut self, dt: std::time::Duration) {
        let dt = ANIMATION_SPEED * dt.as_secs_f32();

        // self.camera_controller.update_camera(&mut self.camera);

        let translation = [0.0, 0.0, 0.0];
        let rotation = [dt.sin(), dt.cos(), 0.0];
        let scaling = [1.0, 1.0, 1.0];

        // let view_mat = self.camera.view_mat();

        // let mvp_mat = self.project_mat * self.view_mat * model_mat;

        let model_mat = transforms::create_transforms(translation, rotation, scaling);
        let view_project_mat = self.project_mat * self.view_mat;
        let normal_mat = (model_mat.invert().unwrap()).transpose();

        let model_ref: &[f32; 16] = model_mat.as_ref();
        let view_projection_ref: &[f32; 16] = view_project_mat.as_ref();
        let normal_ref: &[f32; 16] = normal_mat.as_ref();

        self.init.queue.write_buffer(
            &self.vertex_uniform_buffer,
            0,
            bytemuck::cast_slice(model_ref),
        );
        self.init.queue.write_buffer(
            &self.vertex_uniform_buffer,
            64,
            bytemuck::cast_slice(view_projection_ref),
        );
        self.init.queue.write_buffer(
            &self.vertex_uniform_buffer,
            128,
            bytemuck::cast_slice(normal_ref),
        )
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.init.size = new_size;
            self.init.config.width = new_size.width;
            self.init.config.height = new_size.height;
            self.init
                .surface
                .configure(&self.init.device, &self.init.config);
            self.project_mat = transforms::create_projection(
                new_size.width as f32 / new_size.height as f32,
                IS_PERSPECTIVE,
            );
        }
    }

    pub fn input(&mut self, event: &DeviceEvent) -> bool {
        match event {
            // DeviceEvent::Button {
            //     button: 1, // Left Mouse Button
            //     state,
            // } => {
            //     self.mouse_pressed = *state == ElementState::Pressed;
            //     true
            // }
            // DeviceEvent::MouseMotion { delta } => {
            //     if self.mouse_pressed {
            //         self.camera_controller.mouse_move(delta.0, delta.1);
            //     }
            //     true
            // }
            _ => false,
        }
    }
}
