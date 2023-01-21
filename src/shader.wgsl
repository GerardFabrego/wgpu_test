struct Uniforms {
    mvpMatix: mat4x4<f32>,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec4<f32>,
    // @location(1) color: vec4<f32>
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    // @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(vertex_input: VertexInput) -> VertexOutput {    
    var out : VertexOutput;
    out.position = uniforms.mvpMatix * vertex_input.position;
    // out.color = vertex_input.color;

    return out;
}

@fragment
fn fs_main(vertext_output: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(0.0, 0.0, 0.0, 1.0);
}