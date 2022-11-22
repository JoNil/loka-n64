use crate::ecs::query::Component;
use game_derive::Component;
use n64_math::Vec2;

#[derive(Copy, Clone, Component)]
pub struct Size {
    pub size: Vec2,
}
