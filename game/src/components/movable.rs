use alloc::vec::Vec;
use n64_math::Vec2;
use crate::entity::Entity;
use crate::components::systems;
use crate::impl_system;

#[derive(Copy, Clone)]
pub struct MovableComponent {
    pub entity: Entity,
    pub pos: Vec2,
    pub speed: Vec2,
}

pub fn simulate(dt: f32) {
    for component in lock_mut().components_mut() {
        component.pos += dt * component.speed;
    }
}

impl_system!(MovableComponent);