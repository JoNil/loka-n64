use crate::components::systems;
use crate::entity::Entity;
use crate::impl_system;
use n64_math::Vec2;

#[derive(Debug, Copy, Clone)]
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
