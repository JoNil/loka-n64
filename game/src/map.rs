use crate::{camera::Camera, maps::MAP_1_TILES};
use n64::{gfx::CommandBuffer, VideoMode};
use n64_math::Vec2;

const TILE_SIZE: Vec2 = Vec2::new(32.0 / 320.0, 32.0 / 240.0);

pub struct StaticMapData {
    pub width: i32,
    pub height: i32,
    pub layers: &'static [u8],
}

pub struct Map {
    data: &'static StaticMapData,
}

impl Map {
    pub fn load(data: &'static StaticMapData) -> Self {
        Self { data }
    }

    pub fn render(&self, cb: &mut CommandBuffer, video_mode: VideoMode, camera: &Camera) {
        let tiles_in_layer = (self.data.width * self.data.height) as usize;
        let layer_count = self.data.layers.len() / tiles_in_layer;

        dbg!(self.data.layers.len());
        dbg!(self.data.width);
        dbg!(self.data.height);

        for layer in self.data.layers.chunks_exact(tiles_in_layer) {
            for (index, tile) in layer.iter().enumerate() {

                if *tile == 0 {
                    continue;
                }

                let x = index % (self.data.width as usize);
                let y = index / (self.data.width as usize);

                let pos = Vec2::new(x as f32, y as f32) * TILE_SIZE;

                let half_size = TILE_SIZE / 2.0;

                let upper_left = pos - half_size;
                let lower_right = pos + half_size;

                let screen_size = Vec2::new(video_mode.width() as f32, video_mode.height() as f32);

                cb.add_textured_rect(
                    upper_left * screen_size + camera.pos,
                    lower_right * screen_size + camera.pos,
                    MAP_1_TILES[(*tile - 1) as usize].as_texture(),
                );
            }
        }
    }
}
