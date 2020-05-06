use crate::map::StaticTile;
use n64::gfx::StaticTexture;
use crate::textures::SHIP_2_SMALL;
use n64_math::Vec2;

pub static MAP_1: &'static [&'static [StaticTile]] = &[
    &[
        StaticTile::from_static(Vec2::new(0.7, 0.8), &SHIP_2_SMALL),
        StaticTile::from_static(Vec2::new(0.6, 0.8), &SHIP_2_SMALL),
        StaticTile::from_static(Vec2::new(0.5, 0.8), &SHIP_2_SMALL),
        StaticTile::from_static(Vec2::new(0.4, 0.8), &SHIP_2_SMALL),
        StaticTile::from_static(Vec2::new(0.3, 0.8), &SHIP_2_SMALL),
        StaticTile::from_static(Vec2::new(0.2, 0.8), &SHIP_2_SMALL),
    ],
];

include!(concat!(env!("OUT_DIR"), "/map_includes.rs"));
