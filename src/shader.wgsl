struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {    
    var pos = array<vec2<f32>, 9>(
        vec2<f32>(-0.63, 0.9),
        vec2<f32>(-0.65, 0.2),
        vec2<f32>(-0.2, 0.6),
        vec2<f32>(-0.37, -0.07),
        vec2<f32>(0.05, 0.18),
        vec2<f32>(-0.13, -0.4),
        vec2<f32>(0.3, -0.13),
        vec2<f32>(0.13, -0.64),
        vec2<f32>(0.7, -0.3)
    );

    var color = array<vec3<f32>, 9>(
        vec3<f32>(1.0, 0.0, 0.0),
        vec3<f32>(0.0, 1.0, 0.0),
        vec3<f32>(0.0, 0.0, 1.0),
        vec3<f32>(1.0, 0.0, 0.0),
        vec3<f32>(0.0, 1.0, 0.0),
        vec3<f32>(0.0, 0.0, 1.0),
        vec3<f32>(1.0, 0.0, 0.0),
        vec3<f32>(0.0, 1.0, 0.0),
        vec3<f32>(0.0, 0.0, 1.0)
    );

    var out : VertexOutput;
    out.position = vec4<f32>(pos[in_vertex_index], 0.0, 1.0);
    out.color = color[in_vertex_index];

    return out;
}

@fragment
fn fs_main(vertext_output: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(vertext_output.color, 1.0);
}