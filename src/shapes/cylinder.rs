use cgmath::*;

fn cylinder_position(r: f32, y: f32, theta: Deg<f32>) -> [f32; 3] {
    let x = r * theta.cos();
    let z = -r * theta.sin();
    [x, y, z]
}

pub fn cylinder_data(r_in: f32, r_out: f32, height: f32, n: usize) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 3]>) {
    let h = height / 2.0;
    let capacity = 16 * (n - 1) as usize;
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(capacity);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(capacity);
    let uvs: Vec<[f32; 3]> = Vec::with_capacity(capacity);

    for i in 0..(n - 1) {
        let theta = i as f32 * 360.0 / (n as f32 - 1.0);
        let theta_diff = (i + 1) as f32 * 360.0 / (n as f32 - 1.0);

        let p0 = cylinder_position(r_out, h, Deg(theta));
        let p1 = cylinder_position(r_out, -h, Deg(theta));
        let p2 = cylinder_position(r_in, -h, Deg(theta));
        let p3 = cylinder_position(r_in, h, Deg(theta));
        let p4 = cylinder_position(r_out, h, Deg(theta_diff));
        let p5 = cylinder_position(r_out, -h, Deg(theta_diff));
        let p6 = cylinder_position(r_in, -h, Deg (theta_diff));
        let p7 = cylinder_position(r_in, h, Deg(theta_diff));

        // positions

        // top face
        positions.push(p0);
        positions.push(p4);
        positions.push(p7);
        positions.push(p7);
        positions.push (p3);
        positions.push(p0);

        // bottom face
        positions.push(p1);
        positions.push(p2);
        positions.push(p6);
        positions.push(p6);
        positions.push(p5);
        positions.push(p1);

        // outer face
        positions.push(p0);
        positions.push (p1);
        positions.push(p5);
        positions.push(p5);
        positions.push (p4);
        positions.push(p0);

        // inner face
        positions.push(p2);
        positions.push(p3);
        positions.push(p7);
        positions.push(p7);
        positions.push (p6);
        positions.push(p2);


        // normals
        // top face
        normals.push([0.0, 1.0, 0.0]);
        normals.push([0.0, 1.0, 0.0]);
        normals.push([0.0, 1.0, 0.0]);
        normals.push([0.0, 1.0, 0.0]);
        normals.push([0.0, 1.0, 0.0]);
        normals.push([0.0, 1.0, 0.0]);

        // bottom face
        normals.push([0.0, -1.0, 0.0]);
        normals.push([0.0, -1.0, 0.0]);
        normals.push([0.0, -1.0, 0.0]);
        normals.push([0.0, -1.0, 0.0]);
        normals.push([0.0, -1.0, 0.0]);
        normals.push([0.0, -1.0, 0.0]);

        // outer face
        normals.push([p0[0]/r_out, 0.0, p0[2]/r_out]);
        normals.push([p1[0]/r_out, 0.0, p1[2]/r_out]);
        normals.push([p5[0]/r_out, 0.0, p5[2]/r_out]);
        normals.push([p5[0]/r_out, 0.0, p5[2]/r_out]);
        normals.push([p4[0]/r_out, 0.0, p4[2]/r_out]);
        normals.push([p0[0]/r_out, 0.0, p0[2]/r_out]);


        // inner face
        normals.push([p3[0]/r_in, 0.0, p3[2]/r_in]);
        normals.push([p7[0]/r_in, 0.0, p7[2]/r_in]);
        normals.push([p6[0]/r_in, 0.0, p6[2]/r_in]);
        normals.push([p6[0]/r_in, 0.0, p6[2]/r_in]);
        normals.push([p2[0]/r_in, 0.0, p2[2]/r_in]);
        normals.push([p3[0]/r_in, 0.0, p3[2]/r_in]);

    }
    (positions, normals, uvs)
}
