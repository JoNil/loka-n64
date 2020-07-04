use crate::{camera::Camera, maps::MAP_1_TILES};
use n64::{gfx::CommandBuffer, VideoMode};
use n64_math::Vec2;

pub struct StaticMapData {
    pub width_in_tiles: i32,
    pub height_in_tiles: i32,
    pub tile_width: i32,
    pub tile_height: i32,
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
        let tiles_in_layer = (self.data.width_in_tiles * self.data.height_in_tiles) as usize;

        let tile_scale: Vec2 = Vec2::new(32.0, 32.0);

        let camera_tile =
            camera.pos / Vec2::new(self.data.tile_width as f32, self.data.tile_height as f32);

        let tiles_on_screen_x = (video_mode.width() / self.data.tile_width) + 2;
        let tiles_on_screen_y = (video_mode.height() / self.data.tile_height) + 2;

        let first_tile_x = camera_tile.x() as i32;
        let first_tile_y = camera_tile.y() as i32;

        for layer in self.data.layers.chunks_exact(tiles_in_layer) {
            for y in first_tile_y..(first_tile_y + tiles_on_screen_y) {
                if y < 0 || y >= self.data.height_in_tiles {
                    continue;
                }

                for x in first_tile_x..(first_tile_x + tiles_on_screen_x) {
                    if x < 0 || x >= self.data.width_in_tiles {
                        continue;
                    }

                    let index = x + y * self.data.width_in_tiles;
                    let tile = layer[index as usize];

                    if tile == 0 {
                        continue;
                    }

                    let pos = Vec2::new(
                        (x * self.data.tile_width) as f32,
                        (y * self.data.tile_height) as f32,
                    );

                    let half_size = tile_scale / 2.0;

                    let upper_left = pos - half_size;
                    let lower_right = pos + half_size;

                    cb.add_textured_rect(
                        upper_left - camera.pos,
                        lower_right - camera.pos,
                        MAP_1_TILES[(tile - 1) as usize].as_texture(),
                    );
                }
            }
        }
    }

    pub fn get_start_pos(&self) -> Vec2 {
        Vec2::new(0.0, ((self.data.height_in_tiles - 1) * self.data.tile_height) as f32)
    }
}
