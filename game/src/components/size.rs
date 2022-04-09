use crate::ecs::{entity::Entity, storage::Storage, world::World};
use n64_math::Vec2;

#[derive(Copy, Clone)]
pub struct Size {
    pub size: Vec2,
}

pub fn size(storage: &Storage<Size>, entity: Entity) -> Option<Vec2> {
    storage.lookup(entity).map(|c| c.size)
}
