use n64_math::Vec2;

use crate::ecs::{
    entity::Entity,
    query::{query, Component},
    storage::Storage,
    world::World,
};

#[derive(Copy, Clone)]
pub struct Movable {
    pub pos: Vec2,
    pub speed: Vec2,
}

impl Component for Movable {
    type Inner = Movable;
}

pub fn pos(storage: &Storage<Movable>, entity: Entity) -> Option<Vec2> {
    storage.lookup(entity).map(|c| c.pos)
}

pub fn simulate(world: &mut World, dt: f32) {
    for (_e, movable) in query::<(Movable,)>(&mut world.components) {
        movable.pos += dt * movable.speed;
    }
}
