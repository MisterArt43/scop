pub mod vec2;
pub mod vec3;
pub mod vec4;
pub mod mat4;
pub mod transform;
pub mod quaternion;

pub fn radians(degrees: f32) -> f32 {
	degrees * std::f32::consts::PI / 180.0
}