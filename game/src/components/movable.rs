use crate::components::systems;
use crate::entity::Entity;
use crate::impl_system;
use n64_math::Vec2;

#[derive(Copy, Clone)]
pub struct MovableComponent {
    pub pos: Vec2,
    pub speed: Vec2,
}

pub fn pos(entity: &Entity) -> Option<Vec2> {
    if let Some(movable) = get_component(entity) {
        Some(movable.pos)
    } else {
        None
    }
}

pub fn simulate(dt: f32) {

    for component in lock_mut().components_mut() {
        component.pos += dt * component.speed;
    }
}

impl_system!(MovableComponent);
