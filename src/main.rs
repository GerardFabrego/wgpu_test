mod camera;
mod common;
mod cube;
mod init_wgpu;
mod render;
mod transforms;
mod vertex;

use vertex::*;

fn main() {
    fn create_vertices() -> Vec<Vertex> {
        let p: [[f32; 3]; 8] = [
            [-1.0, 1.0, 1.0],
            [-1.0, 1.0, -1.0],
            [1.0, 1.0, -1.0],
            [1.0, 1.0, 1.0],
            [-1.0, -1.0, 1.0],
            [-1.0, -1.0, -1.0],
            [1.0, -1.0, -1.0],
            [1.0, -1.0, 1.0],
        ];

        // line segments
        let lines: [[f32; 3]; 24] = [
            // 4 lines on top face
            p[0], p[1], p[1], p[2], p[2], p[3], p[3], p[0], // 4 lines on bottom race
            p[4], p[5], p[5], p[6], p[6], p[7], p[7], p[4], // 4 lines on sides
            p[0], p[4], p[1], p[5], p[2], p[6], p[3], p[7],
        ];
        let mut data: Vec<Vertex> = Vec::with_capacity(lines.len());
        for i in 0..lines.len() {
            data.push(vertex(lines[i]));
        }
        data.to_vec()
    }

    common::run(&(create_vertices()));
}
