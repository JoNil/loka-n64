use super::movable::Movable;
use crate::ecs::{query::query, world::World};

pub struct DiverAi;

pub fn update(world: &mut World) {
    for (_e, _diver_ai, movable) in query::<(DiverAi, Movable)>(&mut world.components) {
        movable.speed.y += 0.1;
    }
}
