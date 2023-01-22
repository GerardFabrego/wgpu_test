use std::borrow::Cow;

use crate::camera::*;
use crate::init_wgpu;
use crate::transforms;
use crate::vertex::Vertex;

use bytemuck::cast_slice;
use cgmath::Matrix4;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use winit::{event::*, window::Window};

pub struct Render {
    init: init_wgpu::InitWgpu,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    // index_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,

    num_vertices: u32,
    camera: Camera,
    camera_controller: CameraController,

    project_mat: Matrix4<f32>,
    mouse_pressed: bool,
}

const IS_PERSPECTIVE: bool = true;
const ANIMATION_SPEED: f32 = 1.0;

impl Render {
    pub async fn new(window: &Window, mesh_data: &Vec<Vertex>) -> Self {
        let init = init_wgpu::InitWgpu::init_wgpu(window).await;

        // Create buffers

        let vertex_buffer = init.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: cast_slice(&mesh_data), // bytemuck::cast_slice()
            usage: wgpu::BufferUsages::VERTEX,
        });

        // let index_buffer = init.device.create_buffer_init(&BufferInitDescriptor {
        //     label: Some("Vertex Buffer"),
        //     contents: cast_slice(&index_data),
        //     usage: wgpu::BufferUsages::INDEX,
        // });

        // Load the shaders from disk

        let shader = init
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
            });

        let camera_position = (-5.0, 0.0, 0.0);
        let yaw = cgmath::Deg(0.0);
        let pitch = cgmath::Deg(0.0);
        let speed = 0.005;

        let camera = Camera::new(camera_position, yaw, pitch);
        let camera_controller = CameraController::new(speed);

        let model_mat =
            transforms::create_transforms([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);

        let view_mat = camera.view_mat();

        let project_mat = transforms::create_projection(
            init.config.width as f32 / init.config.height as f32,
            IS_PERSPECTIVE,
        );

        let mvp_mat = project_mat * view_mat * model_mat;
        let mvp_ref: &[f32; 16] = mvp_mat.as_ref();

        // Create uniform buffer

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
                    topology: wgpu::PrimitiveTopology::LineList,
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
            // index_buffer,
            uniform_buffer,
            uniform_bind_group,
            num_vertices: mesh_data.len() as u32,
            camera,
            camera_controller,
            project_mat,
            mouse_pressed: false,
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
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
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
            // render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);

            render_pass.draw(0..self.num_vertices, 0..1);
        }

        self.init.queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }

    pub fn update(&mut self, dt: std::time::Duration) {
        let dt = ANIMATION_SPEED * dt.as_secs_f32();

        self.camera_controller.update_camera(&mut self.camera);

        let translation = [0.0, 0.0, 0.0];
        let rotation = [dt.sin(), dt.cos(), 0.0];
        let scaling = [1.0, 1.0, 1.0];

        let model_mat = transforms::create_transforms(translation, rotation, scaling);
        let view_mat = self.camera.view_mat();

        let mvp_mat = self.project_mat * view_mat * model_mat;
        let mvp_ref: &[f32; 16] = mvp_mat.as_ref();
        self.init
            .queue
            .write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(mvp_ref))
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.init.size = new_size;
        self.init.config.width = new_size.width;
        self.init.config.height = new_size.height;
        self.init
            .surface
            .configure(&self.init.device, &self.init.config);
    }

    pub fn input(&mut self, event: &DeviceEvent) -> bool {
        match event {
            DeviceEvent::Button {
                button: 1, // Left Mouse Button
                state,
            } => {
                self.mouse_pressed = *state == ElementState::Pressed;
                true
            }
            DeviceEvent::MouseMotion { delta } => {
                if self.mouse_pressed {
                    self.camera_controller.mouse_move(delta.0, delta.1);
                }
                true
            }
            _ => false,
        }
    }
}
