use crate::colormap;
use cgmath::*;
use crate::colormap::ColorMapSchemes;

fn normalize_point(
    pt: [f32; 3],
    xmin: f32,
    xmax: f32,
    ymin: f32,
    ymax: f32,
    zmin: f32,
    zmax: f32,
    scale: f32,
) -> [f32; 3] {
    let px = scale * (-1.0 + 2.0 * (pt[0] - xmin) / (xmax - xmin));
    let py = scale * (-1.0 + 2.0 * (pt[1] - ymin) / (ymax - ymin));
    let pz = scale * (-1.0 + 2.0 * (pt[2] - zmin) / (zmax - zmin));

    [px, py, pz]
}

fn create_quad(
    p0: [f32; 3],
    p1: [f32; 3],
    p2: [f32; 3],
    p3: [f32; 3],
    ymin: f32,
    ymax: f32,
    colormap_name: &ColorMapSchemes,
) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 3]>) {
    let position: Vec<[f32; 3]> = vec![p0, p1, p2, p2, p3, p0];

    let ca = Vector3::new(p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]);
    let db = Vector3::new(p3[0] - p1[0], p3[1] - p1[1], p3[2] - p1[2]);
    let cp = (ca.cross(db)).normalize();

    let normal: Vec<[f32; 3]> = vec![
        [cp[0], cp[1], cp[2]],
        [cp[0], cp[1], cp[2]],
        [cp[0], cp[1], cp[2]],
        [cp[0], cp[1], cp[2]],
        [cp[0], cp[1], cp[2]],
        [cp[0], cp[1], cp[2]],
    ];

    let c0 = colormap::color_lerp(colormap_name, ymin, ymax, p0[1]);
    let c1 = colormap::color_lerp(colormap_name, ymin, ymax, p1[1]);
    let c2 = colormap::color_lerp(colormap_name, ymin, ymax, p2[1]);
    let c3 = colormap::color_lerp(colormap_name, ymin, ymax, p3[1]);

    let color: Vec<[f32; 3]> = vec![c0, c1, c2, c2, c3, c0];

    let uv: Vec<[f32; 3]> = Vec::with_capacity(6);
    (position, normal, color, uv)
}

pub fn simple_surface_data(
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
) -> (
    Vec<[f32; 3]>,
    Vec<[f32; 3]>,
    Vec<[f32; 3]>,
    Vec<[f32; 3]>,
    Vec<[f32; 3]>,
) {
    let dx = (xmax - xmin) / (nx as f32 - 1.0);
    let dz = (zmax - zmin) / (nz as f32 - 1.0);
    let mut ymin1: f32 = 0.0;
    let mut ymax1: f32 = 0.0;
    let mut pts: Vec<Vec<[f32; 3]>> = vec![vec![Default::default(); nz]; nx];
    for i in 0..nx {
        let x = xmin + i as f32 * dx;
        let mut pt1: Vec<[f32; 3]> = Vec::with_capacity(nz);
        for j in 0..nz {
            let z = zmin + j as f32 * dz;
            let pt: [f32; 3] = f(x, z);
            pt1.push(pt);
            ymin1 = pt[1];
            ymin1 = if pt[1] < ymin1 { pt[1] } else { ymin1 };
            ymax1 = if pt[1] > ymax1 { pt[1] } else { ymax1 };
        }
        pts[i] = pt1;
    }
    let ymin = ymin1 - scaley * (ymax1 - ymin1);
    let ymax = ymax1 + scaley * (ymax1 - ymin1);

    for i in 0..nx {
        for j in 0..nz {
            pts[i][j] = normalize_point(pts[i][j], xmin, xmax, ymin, ymax, zmin, zmax, scale);
        }
    }

    let cmin = normalize_point([0.0, ymin1, 0.0], xmin, xmax, ymin, ymax, zmin, zmax, scale)[1];
    let cmax = normalize_point([0.0, ymax1, 0.0], xmin, xmax, ymin, ymax, zmin, zmax, scale)[1];

    let mut positions: Vec<[f32; 3]> = Vec::with_capacity((4 * (nx - 1) * (nz - 1)) as usize);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity((4 * (nx - 1) * (nz - 1)) as usize);
    let mut colors: Vec<[f32; 3]> = Vec::with_capacity((4 * (nx - 1) * (nz - 1)) as usize);

    let uvs: Vec<[f32; 3]> = Vec::with_capacity((4 * (nx - 1) * (nz - 1)) as usize);
    let uv1: Vec<[f32; 3]> = Vec::with_capacity((4 * (nx - 1) * (nz - 1)) as usize);

    for i in 0..nx - 1 {
        for j in 0..nz - 1 {
            let p0 = pts[i][j];
            let p1 = pts[i][j + 1];
            let p2 = pts[i + 1][j + 1];
            let p3 = pts[i + 1][j];
            let (mut pos, mut norm, mut col, _uv) =
                create_quad(p0, p1, p2, p3, cmin, cmax, colormap_name);
            // positions
            positions.append(&mut pos);
            normals.append(&mut norm);
            // colors
            colors.append(&mut col);
        }
    }

    (positions, normals, colors, uvs, uv1)
}
