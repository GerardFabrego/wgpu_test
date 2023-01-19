struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(vertex_input: VertexInput) -> VertexOutput {    
    var out : VertexOutput;
    out.position = vec4<f32>(vertex_input.position, 0.0, 1.0);
    out.color = vertex_input.color;

    return out;
}

@fragment
fn fs_main(vertext_output: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(vertext_output.color, 1.0);
}