use n64_math::Vec2;

use crate::ecs::{entity::Entity, storage::Storage, world::World};

#[derive(Copy, Clone)]
pub struct Movable {
    pub pos: Vec2,
    pub speed: Vec2,
}

pub fn pos(movalbe: &Storage<Movable>, entity: Entity) -> Option<Vec2> {
    movalbe.lookup(entity).map(|c| c.pos)
}

pub fn simulate(world: &mut World, dt: f32) {
    let movable = world.components.get::<Movable>();

    for component in movable.components_mut() {
        component.pos += dt * component.speed;
    }
}
