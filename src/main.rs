mod camera;
mod common;
mod cube;
mod init_wgpu;
mod render;
mod transforms;
mod vertex;

use cgmath::*;
use vertex::*;

fn main() {
    fn create_cube_wireframe() -> Vec<Vertex> {
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

    pub fn sphere_position(r: f32, theta: Deg<f32>, phi: Deg<f32>) -> [f32; 3] {
        let tsin = theta.sin();

        let x = r * tsin * phi.cos();
        let y = r * theta.cos();
        let z = -r * tsin * phi.sin();

        [x, y, z]
    }

    fn create_sphere_wireframe(r: f32, u: u32, v: u32) -> Vec<Vertex> {
        let mut points: Vec<Vertex> = Vec::with_capacity((4 * (u - 1) * (v - 1)) as usize);

        for i in 0..(u - 1) {
            for j in 0..(v - 1) {
                let theta = i as f32 * 180.0 / (u as f32 - 1.0);
                let phi = j as f32 * 360.0 / (v as f32 - 1.0);
                let theta_diff = (i as f32 + 1.0) * 180.0 / (u as f32 - 1.0);
                let phi_diff = (j as f32 + 1.0) * 360.0 / (v as f32 - 1.0);

                let p0 = sphere_position(r, Deg(theta), Deg(phi));
                let p1 = sphere_position(r, Deg(theta_diff), Deg(phi));
                let p2 = sphere_position(r, Deg(theta), Deg(phi_diff));

                points.push(vertex(p0));
                points.push(vertex(p1));
                points.push(vertex(p0));
                points.push(vertex(p2));
            }
        }

        points
    }

    common::run(&(create_sphere_wireframe(1.7, 15, 20)));
}
