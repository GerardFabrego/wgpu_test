use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Light {
    specular_color: [f32; 4],
    ambient_intensity: f32,
    diffuse_intensity: f32,
    specular_intensity: f32,
    specular_shininess: f32,
    is_two_side: i32,
}

pub fn light(sc: [f32; 3], ai: f32, di: f32, si: f32, ss: f32, is_two_sized: bool) -> Light {
    Light {
        specular_color: [sc[0], sc[1], sc[2], 1.0],
        ambient_intensity: ai,
        diffuse_intensity: di,
        specular_intensity: si,
        specular_shininess: ss,
        is_two_side: if is_two_sized { 1 } else { 0 },
    }
}
