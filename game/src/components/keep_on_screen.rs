use super::{movable::Movable, size::Size};
use crate::{
    camera::Camera,
    ecs::{
        query::{query, Component},
        world::World,
    },
};
use n64_math::{vec2, Aabb2};

pub struct KeepOnScreen;

impl Component for KeepOnScreen {
    type Inner = KeepOnScreen;
    type RefInner<'w> = &'w mut KeepOnScreen;

    fn convert(v: &mut Self::Inner) -> Self::RefInner<'_> {
        v
    }

    fn empty<'w>() -> Self::RefInner<'w> {
        unreachable!()
    }
}

pub fn update(world: &mut World, camera: &Camera) {
    let camera_bb = Aabb2::new(camera.pos, camera.pos + vec2(1.0, 1.0));

    for (_e, _keep_on_screen, movable, size) in
        query::<(KeepOnScreen, Movable, Size)>(&mut world.components)
    {
        let bb = Aabb2::from_center_size(movable.pos, size.size);

        let outside = camera_bb.outsize_distance(&bb);

        movable.pos -= outside;
    }
}
