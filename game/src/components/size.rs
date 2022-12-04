use game_derive::DenseComponent;
use n64_math::Vec2;

#[derive(Copy, Clone, DenseComponent, Default)]
pub struct Size {
    pub size: Vec2,
}
