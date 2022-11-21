use super::{movable::Movable, size::Size};
use crate::{
    camera::Camera,
    ecs::{entity::EntitySystem, query::query, world::World},
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
    let camera_bb = Aabb2::new(camera.pos, camera.pos + vec2(1.0, 1.0));

    for (e, spawner, movable, size) in query::<(Spawner, Movable, Size)>(&mut world.components) {
        let bb = Aabb2::from_center_size(movable.pos, size.size);

        if camera_bb.collides(&bb) {
            (spawner.spawner_func)(&mut world.entities, *movable, *size, spawner.texture);
            world.entities.despawn(e);
        }
    }
}
