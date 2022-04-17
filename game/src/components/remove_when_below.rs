use super::{movable::Movable, size::Size};
use crate::{camera::Camera, ecs::world::World};
use n64_math::{vec2, Aabb2};

pub struct RemoveWhenBelow;

pub fn update(world: &mut World, camera: &Camera) {
    let (keep_on_screen, movable, size) = world.components.get3::<RemoveWhenBelow, Movable, Size>();

    let camera_bb = Aabb2::new(camera.pos, camera.pos + vec2(1.0, 1.0));

    for entity in keep_on_screen.entities() {
        if let (Some(m), Some(s)) = (movable.lookup(*entity), size.lookup(*entity)) {
            let bb = Aabb2::from_center_size(m.pos, s.size);

            if bb.top() > camera_bb.bottom() {
                world.entities.despawn(*entity);
            }
        }
    }
}
