use crate::{
    components::{movable, sprite_drawable},
    entity::{self, OwnedEntity},
    maps::MAP_1_TILES,
};
use alloc::vec::Vec;
use movable::MovableComponent;
use n64::gfx::StaticTexture;
use n64_math::Vec2;
use sprite_drawable::SpriteDrawableComponent;

const TILE_SIZE: Vec2 = Vec2::new(32.0 / 320.0, 32.0 / 240.0);

pub struct StaticTileDesc {
    pub texture: &'static StaticTexture,
}

pub struct StaticMapData {
    pub width: i32,
    pub height: i32,
    pub layers: &'static [u8],
}

struct Tile {
    entity: OwnedEntity,
}

struct Layer {
    tiles: Vec<Tile>,
}

pub struct Map {
    layers: Vec<Layer>,
}

impl Map {
    pub fn load(data: &'static StaticMapData) -> Self {
        let tiles_in_layer = (data.width * data.height) as usize;
        let layer_count = data.layers.len() / tiles_in_layer;

        let mut layers = Vec::with_capacity(layer_count);

        for layer in data.layers.chunks_exact(tiles_in_layer) {
            let mut tiles = Vec::with_capacity(layer.len());

            for (index, tile) in layer.iter().enumerate() {

                let entity = entity::create();

                let x = index % (data.width as usize);
                let y = index / (data.width as usize);

                let pos = Vec2::new(x as f32, y as f32) * TILE_SIZE;

                movable::add(
                    &entity,
                    MovableComponent {
                        pos: pos,
                        speed: Vec2::zero(),
                    },
                );
                sprite_drawable::add(
                    &entity,
                    SpriteDrawableComponent {
                        size: TILE_SIZE,
                        texture: MAP_1_TILES[*tile as usize].as_texture(),
                    },
                );

                tiles.push(Tile { entity });
            }

            layers.push(Layer { tiles });
        }

        Self { layers }
    }
}
