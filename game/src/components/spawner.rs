use super::{movable::Movable, size::Size};
use crate::{
    camera::Camera,
    ecs::{entity::EntitySystem, world::World},
};
use n64::gfx::Texture;
use n64_math::{vec2, Aabb2};

pub type SpawnerFunc =
    fn(entities: &mut EntitySystem, movable: Movable, size: Size, texture: Texture<'static>);

pub struct Spawner {
    pub texture: Texture<'static>,
    pub spawner_func: SpawnerFunc,
}

pub fn update(world: &mut World, camera: &Camera) {
    let (spawner, movable, size) = world.components.get3::<Spawner, Movable, Size>();

    let camera_bb = Aabb2::new(camera.pos, camera.pos + vec2(1.0, 1.0));

    for (component, entity) in spawner.components_and_entities() {
        if let (Some(m), Some(s)) = (movable.lookup_mut(entity), size.lookup(entity)) {
            let bb = Aabb2::from_center_size(m.pos, s.size);

            if camera_bb.collides(&bb) {
                (component.spawner_func)(&mut world.entities, *m, *s, component.texture);
                world.entities.despawn(entity);
            }
        }
    }
}
