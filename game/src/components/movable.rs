use crate::entity::Entity;
use crate::impl_system;
use n64_math::Vec2;

#[derive(Copy, Clone)]
pub struct MovableComponent {
    pub pos: Vec2,
    pub speed: Vec2,
}

impl System {
    pub fn pos(&self, entity: &Entity) -> Option<Vec2> {
        self.lookup(entity).map(|c| c.pos)
    }

    pub fn simulate(&mut self, dt: f32) {
        for component in self.components_mut() {
            component.pos += dt * component.speed;
        }
    }
}

impl_system!(MovableComponent);
