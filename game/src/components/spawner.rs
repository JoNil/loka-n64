use super::{enemy::spawn_enemy, movable::Movable, size::Size};
use crate::{camera::Camera, ecs::world::World};
use n64::gfx::Texture;
use n64_math::{vec2, Aabb2};

pub struct Spawner {
    pub texture: Texture<'static>,
}

pub fn update(world: &mut World, camera: &Camera) {
    let (spawner, movable, size) = world.components.get3::<Spawner, Movable, Size>();

    let camera_bb = Aabb2::new(camera.pos, camera.pos + vec2(1.0, 1.0));

    for (component, entity) in spawner.components_and_entities() {
        if let (Some(m), Some(s)) = (movable.lookup_mut(entity), size.lookup(entity)) {
            let bb = Aabb2::from_center_size(m.pos, s.size);

            if camera_bb.collides(&bb) {
                spawn_enemy(&mut world.entities, *m, *s, component.texture);
                world.entities.despawn(entity);
            }
        }
    }
}
