mod camera;
mod common;
mod cube;
mod init_wgpu;
mod render;
mod transforms;
mod vertex;

use cube::cube_data;
use vertex::Vertex;

fn main() {
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

    let (vertices, indices) = create_vertices();

    common::run((&vertices, &indices));
}
