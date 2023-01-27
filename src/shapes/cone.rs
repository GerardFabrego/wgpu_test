use cgmath::*;

fn cone_position(r: f32, y: f32, theta: Deg<f32>) -> [f32; 3] {
    let x = r * theta.cos();
    let z = -r * theta.sin();
    [x, y, z]
}

pub fn cone_data(
    r_top: f32,
    r_bottom: f32,
    height: f32,
    n: usize,
) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 3]>) {
    let h = height / 2.0;

    let capacity = 16 * (n - 1);
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(capacity);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(capacity);
    let uvs: Vec<[f32; 3]> = Vec::with_capacity(capacity);

    let inner_down = cone_position(0.0, -h, Deg(0.0));
    let inner_up = cone_position(0.0, h, Deg(0.0));

    for i in 0..(n - 1) {
        let theta = i as f32 * 360.0 / (n as f32 - 1.0);
        let theta_diff = (i + 1) as f32 * 360.0 / (n as f32 - 1.0);

        let theta = i as f32 * 360.0 / (n as f32 - 1.0);
        let theta_diff = (i + 1) as f32 * 360.0 / (n as f32 - 1.0);

        let p0 = cone_position(r_top, h, Deg(theta));
        let p1 = cone_position(r_bottom, -h, Deg(theta));
        let p2 = cone_position(0.0, -h, Deg(theta));
        let p3 = cone_position(0.0, h, Deg(theta));
        let p4 = cone_position(r_top, h, Deg(theta_diff));
        let p5 = cone_position(r_bottom, -h, Deg(theta_diff));

        // top face
        positions.push(p0);
        positions.push(p4);
        positions.push(p3);

        // bottom face
        positions.push(p1);
        positions.push(p2);
        positions.push(p5);

        // outer face
        positions.push(p0);
        positions.push(p1);
        positions.push(p5);
        positions.push(p5);
        positions.push(p4);
        positions.push(p0);

        // normals
        let ca = Vector3::new(p5[0] - p0[0], p5[1] - p0[1], p5[2] - p0[2]);
        let db = Vector3::new(p4[0] - p1[0], p4[1] - p1[1], p4[2] - p1[2]);
        let cp = (ca.cross(db)).normalize();

        // top face
        normals.push([0.0, 1.0, 0.0]);
        normals.push([0.0, 1.0, 0.0]);
        normals.push([0.0, 1.0, 0.0]);
        // bottom face
        normals.push([0.0, -1.0, 0.0]);
        normals.push([0.0, -1.0, 0.0]);
        normals.push([0.0, -1.0, 0.0]);

        // outer face
        normals.push([cp[0], cp[1], cp[2]]);
        normals.push([cp[0], cp[1], cp[2]]);
        normals.push([cp[0], cp[1], cp[2]]);
        normals.push([cp[0], cp[1], cp[2]]);
        normals.push([cp[0], cp[1], cp[2]]);
        normals.push([cp[0], cp[1], cp[2]]);
    }

    (positions, normals, uvs)
}
