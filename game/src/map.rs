use alloc::vec::Vec;
use crate::{components::{sprite_drawable, movable}, entity::{self, OwnedEntity}};
use n64::gfx::StaticTexture;
use n64_math::Vec2;
use movable::MovableComponent;
use sprite_drawable::SpriteDrawableComponent;

const TILE_SIZE: Vec2 = Vec2::new(32.0 / 320.0, 32.0 / 240.0);

#[derive(Copy, Clone)]
pub struct StaticTile {
    pub pos: Vec2,
    pub texture: &'static StaticTexture,
}

impl StaticTile {
    #[inline]
    pub const fn from_static(pos: Vec2, texture: &'static StaticTexture) -> Self {
        Self {
            pos,
            texture,
        }
    }
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
    pub fn load(data: &'static [&'static [StaticTile]]) -> Self {

        let mut layers = Vec::with_capacity(data.len());

        for layer in data {

            let mut tiles = Vec::with_capacity(layer.len());

            for tile in *layer {

                let entity = entity::create();

                movable::add(
                    &entity,
                    MovableComponent {
                        pos: tile.pos,
                        speed: Vec2::zero(),
                    },
                );
                sprite_drawable::add(
                    &entity,
                    SpriteDrawableComponent {
                        size: TILE_SIZE,
                        texture: tile.texture.as_texture(),
                    },
                );

                tiles.push(Tile {
                    entity,
                });
            }

            layers.push(Layer {
                tiles,
            });
        }

        Self {
            layers,
        }
    }
}