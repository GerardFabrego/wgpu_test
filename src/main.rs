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
use crate::shapes::torus::torus_data;


fn create_vertices() -> Vec<Vertex> {
    let (position, normal, _) = torus_data(1.5, 0.4, 25, 25);
    // let (position, normal, _) = sphere_data(1.5, 10, 15);
    let mut mesh = Vec::with_capacity(position.len());
    for i in 0..position.len() {
        mesh.push(vertex(position[i], normal[i]))
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
