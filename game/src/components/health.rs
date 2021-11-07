use crate::{component_storage::Storage, entity::Entity, world::World};

#[derive(Copy, Clone)]
pub struct Health {
    pub health: i32,
}

pub fn damage(health: &mut Storage<Health>, entity: Entity, damage: i32) {
    if let Some(component) = health.lookup_mut(entity) {
        component.health = i32::max(0, component.health - damage);
    }
}

pub fn is_alive(health: &Storage<Health>, entity: Entity) -> bool {
    if let Some(component) = health.lookup(entity) {
        component.health > 0
    } else {
        false
    }
}
