use crate::{entity::Entity, world::World};

#[derive(Copy, Clone)]
pub struct Health {
    pub health: i32,
}

pub fn damage(world: &mut World, entity: Entity, damage: i32) {
    if let Some(component) = world.lookup_mut::<Health>(entity) {
        component.health = i32::max(0, component.health - damage);
    }
}

pub fn is_alive(world: &World, entity: Entity) -> bool {
    if let Some(component) = world.lookup::<Health>(entity) {
        component.health > 0
    } else {
        false
    }
}
