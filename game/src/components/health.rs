use crate::entity::Entity;
use crate::impl_system;

#[derive(Copy, Clone)]
pub struct HealthComponent {
    pub health: i32,
}

pub fn damage(entity: &Entity, damage: i32) {
    if let Some(component) = lock_mut().lookup_mut(entity) {
        component.health = i32::max(0, component.health - damage);
    }
}

pub fn is_alive(entity: &Entity) -> bool {
    if let Some(component) = lock().lookup(entity) {
        component.health > 0
    } else {
        false
    }
}

impl_system!(HealthComponent);
