use super::{movable::Movable, size::Size};
use crate::{
    camera::Camera,
    ecs::{entity::EntitySystem, query::query, world::World},
    model::{ModelData, StaticModelData},
};
use game_derive::SparseComponent;
use n64::gfx::{StaticTexture, Texture};
use n64_math::{vec2, Aabb2};

#[derive(Copy, Clone)]
pub enum SpawnerData {
    SpawnerWithModel {
        spawner_func: SpawnerWithModelFunc,
        model: &'static StaticModelData,
    },
    SpawnerWithTexture {
        spawner_func: SpawnerWithTextureFunc,
        texture: &'static StaticTexture,
    },
}

impl SpawnerData {
    pub fn size(&self) -> Size {
        match self {
            SpawnerData::SpawnerWithModel { model, .. } => Size { size: model.size },
            SpawnerData::SpawnerWithTexture { texture, .. } => Size {
                size: vec2(texture.width as f32 / 320.0, texture.height as f32 / 240.0),
            },
        }
    }
}

pub type SpawnerWithModelFunc =
    fn(entities: &mut EntitySystem, movable: Movable, size: Size, model: ModelData<'static>);
pub type SpawnerWithTextureFunc =
    fn(entities: &mut EntitySystem, movable: Movable, size: Size, texture: Texture<'static>);

#[derive(SparseComponent)]
pub struct Spawner {
    pub data: SpawnerData,
}

pub fn update(world: &mut World, camera: &Camera) {
    let camera_bb = Aabb2::new(camera.pos, camera.pos + vec2(1.0, 1.0));

    for (e, spawner, movable, size) in query::<(Spawner, Movable, Size)>(&mut world.components) {
        let bb = Aabb2::from_center_size(movable.pos, size.size);

        if camera_bb.collides(&bb) {
            match &spawner.data {
                SpawnerData::SpawnerWithModel {
                    spawner_func,
                    model,
                } => {
                    spawner_func(&mut world.entities, *movable, *size, model.as_model_data());
                }
                SpawnerData::SpawnerWithTexture {
                    spawner_func,
                    texture,
                } => {
                    spawner_func(&mut world.entities, *movable, *size, texture.as_texture());
                }
            }
            world.entities.despawn(e);
        }
    }
}
