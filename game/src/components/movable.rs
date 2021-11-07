use crate::{entity::Entity, world::World};
use n64_math::Vec2;

#[derive(Copy, Clone)]
pub struct Movable {
    pub pos: Vec2,
    pub speed: Vec2,
}

pub fn pos(world: &World, entity: Entity) -> Option<Vec2> {
    world.lookup::<Movable>(entity).map(|c| c.pos)
}

pub fn simulate(world: &mut World, dt: f32) {
    for component in world.components_mut::<Movable>() {
        component.pos += dt * component.speed;
    }
}
