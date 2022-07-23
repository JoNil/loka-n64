use super::movable::Movable;
use crate::ecs::world::World;

pub struct DiverAi {}

pub fn update(world: &mut World, dt: f32) {
    let (diver_ai, movable) = world.components.get2::<DiverAi, Movable>();

    for (ai, entity) in diver_ai.components_and_entities_mut() {
        if let Some(movable) = movable.lookup_mut(entity) {
            movable.speed.y += 0.1;
        }
    }
}
