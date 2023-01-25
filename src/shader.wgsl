struct LightUniforms {
    color: vec4<f32>,
    specular_color: vec4<f32>,
    ambient_intensity: f32,
    diffuse_intensity :f32,
    specular_intensity: f32,
    specular_shininess: f32
}

@group(0) @binding(2) var<uniform> light_uniforms: LightUniforms;

struct VertexUniforms {
    model_mat: mat4x4<f32>,
    view_project_mat: mat4x4<f32>,
    normal_mat: mat4x4<f32>
}

@group(0) @binding(0) var<uniform> uniforms: VertexUniforms;

struct VertexInput {
    @location(0) position: vec4<f32>,
    @location(1) normal: vec4<f32>
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) v_position: vec4<f32>,
    @location(1) v_normal: vec4<f32>,
}

@vertex
fn vs_main(vertex_input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    let position: vec4<f32> = uniforms.model_mat * vertex_input.position;
    output.position = uniforms.view_project_mat * position;
    output.v_normal = uniforms.normal_mat * vertex_input.normal;
    output.v_position = position;
     return output;
}


struct FragmentUniforms {
    light_position: vec4<f32>,
    eye_position: vec4<f32>
}
@group(0) @binding(1) var<uniform> frag_uniforms : FragmentUniforms;

@fragment
fn fs_main(vertex_output: VertexOutput) -> @location(0) vec4<f32> {
     let N: vec3<f32> = normalize(vertex_output.v_normal.xyz);
     let L: vec3<f32> = normalize(frag_uniforms.light_position.xyz - vertex_output.v_position.xyz);
     let V: vec3<f32> = normalize(frag_uniforms.eye_position.xyz - vertex_output.v_position.xyz);
     let H: vec3<f32> = normalize(L + V);
     let diffuse: f32 = light_uniforms.diffuse_intensity * max(dot(N, L,), 0.0);
     let specular: f32 = light_uniforms.specular_intensity *
        pow(max(dot(N, H),0.0), light_uniforms.specular_shininess);
     let ambient:f32 = light_uniforms.ambient_intensity;
     return vec4(light_uniforms.color.xyz * (ambient + diffuse) + light_uniforms.specular_color.xyz * specular, 1.0);
}