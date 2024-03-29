use crate::ecs::{
    component::Component, entity::Entity, query::query, storage::Storage, world::World,
};
use game_derive::DenseComponent;
use n64_math::Vec2;

#[derive(Copy, Clone, DenseComponent, Default)]
pub struct Movable {
    pub pos: Vec2,
    pub speed: Vec2,
}

pub fn pos(storage: &<Movable as Component>::Storage, entity: Entity) -> Option<Vec2> {
    storage.lookup(entity).map(|c| c.pos)
}

pub fn simulate(world: &mut World, dt: f32) {
    for (_e, movable) in query::<(Movable,)>(&mut world.components) {
        movable.pos += dt * movable.speed;
    }
}
