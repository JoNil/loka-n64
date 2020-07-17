use crate::entity::Entity;
use crate::impl_system;

#[derive(Copy, Clone)]
pub struct HealthComponent {
    pub health: i32,
}

impl System {
    pub fn damage(&mut self, entity: &Entity, damage: i32) {
        if let Some(component) = self.lookup_mut(entity) {
            component.health = i32::max(0, component.health - damage);
        }
    }

    pub fn is_alive(&self, entity: &Entity) -> bool {
        if let Some(component) = self.lookup(entity) {
            component.health > 0
        } else {
            false
        }
    }
}

impl_system!(HealthComponent);
