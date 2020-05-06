use crate::map::{StaticTileDesc, StaticMapData};
use crate::textures::SHIP_2_SMALL;
use n64::gfx::StaticTexture;
use n64_math::Vec2;

pub static TILES: &'static [&'static StaticTexture] = &[
    &SHIP_2_SMALL,
];

pub static TEST_MAP: &'static StaticMapData = &StaticMapData {
    width: 3,
    height: 1,
    layers: &[0, 0, 0],
};

include!(concat!(env!("OUT_DIR"), "/map_includes.rs"));
