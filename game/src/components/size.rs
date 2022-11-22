use n64_math::Vec2;

use crate::ecs::query::Component;

#[derive(Copy, Clone)]
pub struct Size {
    pub size: Vec2,
}

impl Component for Size {
    type Inner = Size;
    type RefInner<'w> = &'w mut Size;

    fn convert(v: &mut Self::Inner) -> Self::RefInner<'_> {
        v
    }

    fn empty<'w>() -> Self::RefInner<'w> {
        unreachable!()
    }
}
