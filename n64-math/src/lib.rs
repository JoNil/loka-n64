#![no_std]

mod aabb2;
mod color;
mod hash;

pub mod rand;

pub use aabb2::Aabb2;
pub use color::Color;
pub use glam::{
    const_vec2, const_vec3, const_vec4, vec2, vec3, vec4, Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec4,
};
pub use hash::{BuildFnvHasher, FnvHasher};
pub use rand::{random_f32, random_f64, random_u32, random_u64};
