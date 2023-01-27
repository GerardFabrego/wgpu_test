mod camera;
mod common;
mod init_wgpu;
mod light;
mod render;
mod shapes;
mod transforms;
mod vertex;

use crate::light::light;
use crate::shapes::cone::cone_data;
use crate::shapes::cylinder::cylinder_data;
use shapes::cube::cube_data;
use shapes::sphere::sphere_data;
use vertex::*;

// fn create_torus_wireframe(r_torus: f32, r_tube: f32, n_torus: u32, n_tube: u32) -> Vec<Vertex> {
//     let mut points: Vec<Vertex> = Vec::with_capacity((4 * n_torus * n_tube) as usize);

//     fn torus_position(r_torus: f32, r_tube: f32, u: Deg<f32>, v: Deg<f32>) -> [f32; 3] {
//         let x = (r_torus + r_tube * v.cos()) * u.cos();
//         let y = r_tube * v.sin();
//         let z = -(r_torus + r_tube * v.cos()) * u.sin();
//         [x, y, z]
//     }

//     for i in 0..(n_torus - 1) {
//         for j in 0..(n_tube - 1) {
//             let u = i as f32 * 360.0 / (n_torus as f32 - 1.0);
//             let v = j as f32 * 360.0 / (n_tube as f32 - 1.0);
//             let u_diff = (i as f32 + 1.0) * 360.0 / (n_torus as f32 - 1.0);
//             let v_diff = (j as f32 + 1.0) * 360.0 / (n_tube as f32 - 1.0);

//             let p0 = torus_position(r_torus, r_tube, Deg(u), Deg(v));
//             let p1 = torus_position(r_torus, r_tube, Deg(u_diff), Deg(v));
//             let p2 = torus_position(r_torus, r_tube, Deg(u), Deg(v_diff));

//             points.push(vertex(p0));
//             points.push(vertex(p1));
//             points.push(vertex(p0));
//             points.push(vertex(p2));
//         }
//     }
//     points
// }

fn create_vertices() -> Vec<Vertex> {
    let (position, normal, _) = cone_data(0.0, 1.5, 3.0, 25);
    let mut mesh = Vec::with_capacity(position.len());
    for i in 0..position.len() {
        mesh.push(vertex(position[i], normal[i]))
    }
    mesh.to_vec()
}

fn main() {
    // let mesh = create_torus_wireframe(1.0, 0.4, 30, 20);

    // Create mesh data
    let mesh = create_vertices();

    // Create light data
    let light = light([1.0, 0.0, 0.0], [1.0, 1.0, 0.0], 0.1, 0.6, 0.3, 30.0);

    common::run(&mesh, light);
}
