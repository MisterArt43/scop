pub mod vec2;
pub mod vec3;
pub mod vec4;
pub mod mat4;
pub mod transform;
pub mod quaternion;

use vec2::Vec2;
use vec3::Vec3;
use vec4::Vec4;
use mat4::Mat4;
use transform::Transform;
use quaternion::Quaternion;


pub fn radians(degrees: f32) -> f32 {
	degrees * std::f32::consts::PI / 180.0
}