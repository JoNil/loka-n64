use super::movable::Movable;
use crate::ecs::world::World;

pub struct PrintPosition;

pub fn print(world: &mut World) {
    let (movable, print_posistion) = world.components.get::<(Movable, PrintPosition)>();

    for entity in print_posistion.entities() {
        if let Some(m) = movable.lookup(*entity) {
            n64::debugln!("Entity: {}, Position: {:?}", entity.index(), m.pos);
        }
    }
}
