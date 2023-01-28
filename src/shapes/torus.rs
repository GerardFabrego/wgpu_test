use cgmath::*;

pub fn torus_data(r_torus: f32, r_tube: f32, n_torus: u32, n_tube: u32) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 3]>) {
        let mut positions = Vec::with_capacity((4 * n_torus * n_tube) as usize);
    let mut normals = Vec::with_capacity((4 * n_torus * n_tube) as usize);
    let uvs = Vec::with_capacity((4 * n_torus * n_tube) as usize);

    fn torus_position(r_torus: f32, r_tube: f32, u: Deg<f32>, v: Deg<f32>) -> [f32; 3] {
        let x = (r_torus + r_tube * v.cos()) * u.cos();
        let y = r_tube * v.sin();
        let z = -(r_torus + r_tube * v.cos()) * u.sin();
        [x, y, z]
    }

    for i in 0..(n_torus - 1) {
        for j in 0..(n_tube - 1) {
            let u = i as f32 * 360.0 / (n_torus as f32 - 1.0);
            let v = j as f32 * 360.0 / (n_tube as f32 - 1.0);
            let u_diff = (i as f32 + 1.0) * 360.0 / (n_torus as f32 - 1.0);
            let v_diff = (j as f32 + 1.0) * 360.0 / (n_tube as f32 - 1.0);

            let p0 = torus_position(r_torus, r_tube, Deg(u), Deg (v));
            let p1 = torus_position(r_torus, r_tube, Deg(u_diff), Deg(v));
            let p2 = torus_position(r_torus, r_tube, Deg(u_diff), Deg(v_diff));
            let p3 = torus_position(r_torus, r_tube, Deg(u), Deg(v_diff));

            // positions
            positions.push (p0);
            positions.push(p1);
            positions.push(p2);
            positions.push(p2);
            positions.push (p3);
            positions.push( p0);

            // normals

            let ca = Vector3::new(p2[0]-p0[0], p2[1]-p0[1], p2[2]-p0[2]);
            let db = Vector3::new(p3[0]-p1[0], p3[1]-p1[1], p3[2]-p1[2]);
            let cp = (ca.cross(db)) .normalize();

            normals.push([cp[0], cp[1], cp[2]]);
            normals.push([cp[0], cp[1], cp[2]]);
            normals.push([cp[0], cp[1], cp[2]]);
            normals.push([cp[0], cp[1], cp[2]]);
            normals.push([cp[0], cp[1], cp[2]]);
            normals.push([cp[0], cp[1], cp[2]]);
        }
    }
    (positions, normals, uvs)
}
