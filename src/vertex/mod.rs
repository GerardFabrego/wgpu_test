use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 4],
    // pub color: [f32; 4],
    pub normal: [f32; 4],
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 1,
                },
                // wgpu::VertexAttribute {
                //     format: wgpu::VertexFormat::Float32x4,
                //     offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                //     shader_location: 2,
                // },
            ],
        }
    }
}

pub fn vertex(position: [f32; 3], normal: [f32; 3]) -> Vertex {
    Vertex {
        position: [position[0], position[1], position[2], 1.0],
        // color: [color[0] as f32, color[1] as f32, color[2] as f32, 1.0],
        normal: [normal[0] as f32, normal[1] as f32, normal[2] as f32, 1.0],
    }
}
