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
use crate::surface::simple_surface_data;
use shapes::cube::cube_data;
use shapes::sphere::sphere_data;
use vertex::*;
use std::mem::size_of_val;
use crate::colormap::ColorMapSchemes;

fn create_vertices(
    f: &dyn Fn(f32, f32) -> [f32; 3],
    colormap_name: &ColorMapSchemes,
    xmin: f32,
    xmax: f32,
    zmin: f32,
    zmax: f32,
    nx: usize,
    nz: usize,
    scale: f32,
    scaley: f32,
) -> Vec<Vertex> {
    let (position, normal, color, _, _) = simple_surface_data(
        f,
        colormap_name,
        xmin,
        xmax,
        zmin,
        zmax,
        nx,
        nz,
        scale,
        scaley,
    );
    // let (position, normal, _) = sphere_data(1.5, 10, 15);
    let mut mesh = Vec::with_capacity(position.len());
    for i in 0..position.len() {
        mesh.push(vertex(position[i], normal[i], color[i]))
    }
    mesh.to_vec()
}

pub fn sinc(x: f32, z: f32) -> [f32; 3] {
    let r = (x * x + z * z).sqrt();
    let y = if r == 0.0 { 1.0 } else { r.sin() / r };
    [x, y, z]
}

fn main() {
    // Create mesh data
    let mesh = create_vertices(&sinc, &ColorMapSchemes::autumn, -8.0, 8.0, -8.0, 8.0, 30, 30, 2.0, 0.3);

    // Create light data
    let light = light([1.0, 1.0, 0.0], 0.1, 0.6, 0.3, 30.0, false);

    // println!("{}", size_of_val(&light));
    common::run(&mesh, light);
}
