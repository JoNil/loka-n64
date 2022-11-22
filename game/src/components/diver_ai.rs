use super::movable::Movable;
use crate::ecs::{
    query::{query, Component},
    world::World,
};

pub struct DiverAi;

impl Component for DiverAi {
    type Inner = DiverAi;
    type RefInner<'w> = &'w mut DiverAi;

    fn convert<'w>(v: &'w mut Self::Inner) -> Self::RefInner<'w> {
        v
    }

    fn empty<'w>() -> Self::RefInner<'w> {
        unreachable!()
    }
}

pub fn update(world: &mut World) {
    for (_e, _diver_ai, movable) in query::<(DiverAi, Movable)>(&mut world.components) {
        movable.speed.y += 0.1;
    }
}
