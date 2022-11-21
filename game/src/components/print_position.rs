use super::movable::Movable;
use crate::ecs::{query::query, world::World};

pub struct PrintPosition;

pub fn print(world: &mut World) {
    for (e, _, movable) in query::<(PrintPosition, Movable)>(&mut world.components) {
        n64::debugln!("Entity: {}, Position: {:?}", e.index(), movable.pos);
    }
}
