use super::movable::Movable;
use crate::ecs::{
    query::{query, Component},
    world::World,
};

pub struct PrintPosition;

impl Component for PrintPosition {
    type Inner = PrintPosition;
    type RefInner<'w> = &'w mut PrintPosition;

    fn convert<'w>(v: &'w mut Self::Inner) -> Self::RefInner<'w> {
        v
    }
}

pub fn print(world: &mut World) {
    for (e, _, movable) in query::<(PrintPosition, Movable)>(&mut world.components) {
        n64::debugln!("Entity: {}, Position: {:?}", e.index(), movable.pos);
    }
}
