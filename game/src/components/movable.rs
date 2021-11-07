use crate::entity::Entity;
use crate::impl_component;
use n64_math::Vec2;

#[derive(Copy, Clone)]
pub struct MovableComponent {
    pub pos: Vec2,
    pub speed: Vec2,
}

impl_component!(MovableComponent);

pub fn pos(movable: &Storage, entity: Entity) -> Option<Vec2> {
    movable.lookup(entity).map(|c| c.pos)
}

pub fn simulate(movable: &mut Storage, dt: f32) {
    for component in movable.components_mut() {
        component.pos += dt * component.speed;
    }
}
