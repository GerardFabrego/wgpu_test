pub fn cube_data() -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 3]>) {
    #[rustfmt::skip]
    let positions = [
        // front (0.0, 0.0, 1.0)
        [-1.0, -1.0, 1.0], [1.0, -1.0, 1.0], [-1.0, 1.0, 1.0], [-1.0, 1.0, 1.0], [1.0, -1.0, 1.0], [1.0, 1.0, 1.0],
        // right (1.0, 0.0, 0.0)
        [1.0, -1.0, 1.0], [1.0, -1.0, -1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, -1.0, -1.0], [1.0, 1.0, -1.0],
        // back (0.0, 0.0, -1.0)
        [1.0, -1.0, -1.0], [-1.0, -1.0, -1.0], [1.0, 1.0, -1.0], [1.0, 1.0, -1.0], [-1.0, -1.0, -1.0], [-1.0, 1.0, -1.0],
        // left (-1.0, 0.0, 0.0)
        [-1.0, -1.0, -1.0], [-1.0, -1.0, 1.0], [-1.0, 1.0, -1.0], [-1.0, 1.0, -1.0], [-1.0, -1.0, 1.0], [-1.0, 1.0, 1.0],
        // top (0.0, 1.0, 0.0)
        [-1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [-1.0, 1.0, -1.0], [-1.0, 1.0, -1.0], [1.0, 1.0, 1.0], [1.0, 1.0, -1.0],
        // bottom (0.0, -1.0, 0.0)
        [-1.0, -1.0, -1.0], [1.0, -1.0, -1.0], [-1.0, -1.0, 1.0], [-1.0, -1.0, 1.0], [1.0, -1.0, -1.0], [1.0, -1.0, 1.0],
    ]
    .to_vec();

    #[rustfmt::skip]
    let colors = [
        // front - blue
        [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0],
        // right - red
        [1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0],
        // back - yellow
        [1.0, 1.0, 0.0], [1.0, 1.0, 0.0], [1.0, 1.0, 0.0], [1.0, 1.0, 0.0], [1.0, 1.0, 0.0], [1.0, 1.0, 0.0],
        // left - aqua
        [0.0, 1.0, 1.0], [0.0, 1.0, 1.0], [0.0, 1.0, 1.0], [0.0, 1.0, 1.0], [0.0, 1.0, 1.0], [0.0, 1.0, 1.0],
        // top - green
        [0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0, 0.0],
        // bottom - fuchsia
        [1.0, 0.0, 1.0], [1.0, 0.0, 1.0], [1.0, 0.0, 1.0], [1.0, 0.0, 1.0], [1.0, 0.0, 1.0], [1.0, 0.0, 1.0],
    ].to_vec();

    // let uvs = [
    //     // front
    //     [0.0, 0.0],
    //     [1.0, 0.0],
    //     [0.0, 1.0],
    //     [0.0, 1.0],
    //     [1.0, 0.0],
    //     [1.0, 1.0],
    //     // right
    //     [0.0, 0.0],
    //     [1.0, 0.0],
    //     [0.0, 1.0],
    //     [0.0, 1.0],
    //     [1.0, 0.0],
    //     [1.0, 1.0],
    //     // back
    //     [0.0, 0.0],
    //     [1.0, 0.0],
    //     [0.0, 1.0],
    //     [0.0, 1.0],
    //     [1.0, 0.0],
    //     [1.0, 1.0],
    //     // left
    //     [0.0, 0.0],
    //     [1.0, 0.0],
    //     [0.0, 1.0],
    //     [0.0, 1.0],
    //     [1.0, 0.0],
    //     [1.0, 1.0],
    //     // top
    //     [0.0, 0.0],
    //     [1.0, 0.0],
    //     [0.0, 1.0],
    //     [0.0, 1.0],
    //     [1.0, 0.0],
    //     [1.0, 1.0],
    //     // bottom
    //     [0.0, 0.0],
    //     [1.0, 0.0],
    //     [0.0, 1.0],
    //     [0.0, 1.0],
    //     [1.0, 0.0],
    //     [1.0, 1.0],
    // ];

    let normals = [
        // front
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        // right
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        // back
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        // left
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        // top
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        // bottom
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
    ].to_vec();

    (positions, normals, colors)
}

// fn create_cube_wireframe() -> Vec<Vertex> {
//     let p: [[f32; 3]; 8] = [
//         [-1.0, 1.0, 1.0],
//         [-1.0, 1.0, -1.0],
//         [1.0, 1.0, -1.0],
//         [1.0, 1.0, 1.0],
//         [-1.0, -1.0, 1.0],
//         [-1.0, -1.0, -1.0],
//         [1.0, -1.0, -1.0],
//         [1.0, -1.0, 1.0],
//     ];
//
//     // line segments
//     let lines: [[f32; 3]; 24] = [
//         // 4 lines on top face
//         p[0], p[1], p[1], p[2], p[2], p[3], p[3], p[0], // 4 lines on bottom race
//         p[4], p[5], p[5], p[6], p[6], p[7], p[7], p[4], // 4 lines on sides
//         p[0], p[4], p[1], p[5], p[2], p[6], p[3], p[7],
//     ];
//     let mut data: Vec<Vertex> = Vec::with_capacity(lines.len());
//     for i in 0..lines.len() {
//         data.push(vertex(lines[i]));
//     }
//     data.to_vec()
// }