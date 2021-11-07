use crate::entity::Entity;
use crate::impl_component;

#[derive(Copy, Clone)]
pub struct HealthComponent {
    pub health: i32,
}

impl_component!(HealthComponent);

pub fn damage(health: &mut Storage, entity: Entity, damage: i32) {
    if let Some(component) = health.lookup_mut(entity) {
        component.health = i32::max(0, component.health - damage);
    }
}

pub fn is_alive(health: &Storage, entity: Entity) -> bool {
    if let Some(component) = health.lookup(entity) {
        component.health > 0
    } else {
        false
    }
}
