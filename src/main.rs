mod camera;
mod colormap;
mod common;
mod init_wgpu;
mod light;
mod render;
mod shapes;
mod surface;
mod transforms;
mod vertex;

use crate::light::light;
use crate::shapes::cone::cone_data;
use crate::shapes::cylinder::cylinder_data;
use crate::shapes::torus::torus_data;
use shapes::cube::cube_data;
use shapes::sphere::sphere_data;
use vertex::*;

fn create_vertices() -> Vec<Vertex> {
    let (position, normal, color) = cube_data();
    // let (position, normal, _) = sphere_data(1.5, 10, 15);
    let mut mesh = Vec::with_capacity(position.len());
    for i in 0..position.len() {
        mesh.push(vertex(position[i], normal[i], color[i]))
    }
    mesh.to_vec()
}

fn main() {
    // Create mesh data
    let mesh = create_vertices();

    // Create light data
    let light = light([1.0, 0.0, 0.0], [1.0, 1.0, 0.0], 0.1, 0.6, 0.3, 30.0);

    common::run(&mesh, light);
}
