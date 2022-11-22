use crate::ecs::{entity::Entity, query::query, storage::Storage, world::World};
use game_derive::Component;

#[derive(Component)]
pub struct Health {
    pub health: i32,
    pub damaged_this_frame: bool,
}

impl Health {
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
}

pub fn damage(health: &mut Storage<Health>, entity: Entity, damage: i32) {
    if let Some(component) = health.lookup_mut(entity) {
        component.health = i32::max(0, component.health - damage);
        component.damaged_this_frame = true;
    }
}

pub fn is_alive(health: &Storage<Health>, entity: Entity) -> bool {
    if let Some(component) = health.lookup(entity) {
        component.is_alive()
    } else {
        false
    }
}

pub fn clear_was_damaged(world: &mut World) {
    for (_e, health) in query::<(Health,)>(&mut world.components) {
        health.damaged_this_frame = false;
    }
}
