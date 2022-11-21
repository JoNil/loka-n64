use crate::ecs::{entity::Entity, storage::Storage, world::World};

pub struct Health {
    pub health: i32,
    pub damaged_this_frame: bool,
}

pub fn damage(health: &mut Storage<Health>, entity: Entity, damage: i32) {
    if let Some(component) = health.lookup_mut(entity) {
        component.health = i32::max(0, component.health - damage);
        component.damaged_this_frame = true;
    }
}

pub fn is_alive(health: &Storage<Health>, entity: Entity) -> bool {
    if let Some(component) = health.lookup(entity) {
        component.health > 0
    } else {
        false
    }
}

pub fn clear_was_damaged(world: &mut World) {
    let health = world.components.get::<(Health,)>();

    for component in health.components_mut() {
        component.damaged_this_frame = false;
    }
}
