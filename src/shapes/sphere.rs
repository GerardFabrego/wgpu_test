use cgmath::*;

fn sphere_position(r: f32, theta: Deg<f32>, phi: Deg<f32>) -> [f32; 3] {
    let tsin = theta.sin();

    let x = r * tsin * phi.cos();
    let y = r * theta.cos();
    let z = -r * tsin * phi.sin();

    [x, y, z]
}

pub fn sphere_data(r: f32, u: usize, v: usize) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 3]>) {
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity((4 * (u - 1) * (v - 1)) as usize);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity((4 * (u - 1) * (v - 1)) as usize);
    let uvs: Vec<[f32; 3]> = Vec::with_capacity((4 * (u - 1) * (v - 1)) as usize);

    for i in 0..u - 1 {
        for j in 0..v - 1 {
            let theta = i as f32 * 180.0 / (u as f32 - 1.0);
            let phi = j as f32 * 360.0 / (v as f32 - 1.0);
            let theta1 = (i as f32 + 1.0) * 180.0 / (u as f32 - 1.0);
            let phil = (j as f32 + 1.0) * 360.0 / (v as f32 - 1.0);
            let p0 = sphere_position(r, Deg(theta), Deg(phi));
            let p1 = sphere_position(r, Deg(theta1), Deg(phi));
            let p2 = sphere_position(r, Deg(theta1), Deg(phil));
            let p3 = sphere_position(r, Deg(theta), Deg(phil));

            // positions
            positions.push(p0);
            positions.push(p1);
            positions.push(p3);
            positions.push(p1);
            positions.push(p2);
            positions.push(p3);

            // normals
            normals.push([p0[0], p0[1], p0[2]]);
            normals.push([p1[0], p1[1], p1[2]]);
            normals.push([p3[0], p3[1], p3[2]]);
            normals.push([p1[0], p1[1], p1[2]]);
            normals.push([p2[0], p2[1], p2[2]]);
            normals.push([p3[0], p3[1], p3[2]]);
        }
    }

    (positions, normals, uvs)
}

// fn create_sphere_wireframe(r: f32, u: u32, v: u32) -> Vec<Vertex> {

//     let mut points: Vec<Vertex> = Vec::with_capacity((4 * (u - 1) * (v - 1)) as usize);

//     for i in 0..(u - 1) {
//         for j in 0..(v - 1) {
//             let theta = i as f32 * 180.0 / (u as f32 - 1.0);
//             let phi = j as f32 * 360.0 / (v as f32 - 1.0);
//             let theta_diff = (i as f32 + 1.0) * 180.0 / (u as f32 - 1.0);
//             let phi_diff = (j as f32 + 1.0) * 360.0 / (v as f32 - 1.0);

//             let p0 = sphere_position(r, Deg(theta), Deg(phi));
//             let p1 = sphere_position(r, Deg(theta_diff), Deg(phi));
//             let p2 = sphere_position(r, Deg(theta), Deg(phi_diff));

//             points.push(vertex(p0));
//             points.push(vertex(p1));
//             points.push(vertex(p0));
//             points.push(vertex(p2));
//         }
//     }

//     points
// }
