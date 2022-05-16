use crate::{camera::Camera, components::enemy::add_enemy, ecs::world::World};
use n64::{
    gfx::{
        color_combiner_mode::{ColorCombinerMode, DSrc},
        CommandBuffer, Pipeline, StaticTexture,
    },
    VideoMode,
};
use n64_math::Vec2;

pub struct StaticObject {
    pub x: f32,
    pub y: f32,
    pub texture: &'static StaticTexture,
}

pub struct StaticMapData {
    pub width_in_tiles: i32,
    pub height_in_tiles: i32,
    pub tile_width: i32,
    pub tile_height: i32,
    pub tiles: &'static [&'static StaticTexture],
    pub layers: &'static [u8],
    pub objects: &'static [&'static [StaticObject]],
}

static MAP_PIPELINE: Pipeline = Pipeline {
    color_combiner_mode: ColorCombinerMode::single(DSrc::Texel),
    blend: true,
    ..Pipeline::default()
};

pub struct Map {
    data: &'static StaticMapData,
}

impl Map {
    pub fn load(data: &'static StaticMapData) -> Self {
        Self { data }
    }

    pub fn spawn_enemies(&self, world: &mut World, video_mode: &VideoMode) {
        for objects in self.data.objects {
            for object in *objects {
                add_enemy(
                    &mut world.entities,
                    Vec2::new(
                        object.x / video_mode.width() as f32,
                        object.y / video_mode.height() as f32,
                    ),
                    object.texture.as_texture(),
                );
            }
        }
    }

    pub fn render(&self, cb: &mut CommandBuffer, video_mode: VideoMode, camera: &Camera) {
        n64::scope!("map::render");

        let tiles_in_layer = (self.data.width_in_tiles * self.data.height_in_tiles) as usize;

        let tile_scale: Vec2 = Vec2::new(32.0, 32.0);

        let camera_pixel_pos = Vec2::new(
            camera.pos.x * video_mode.width() as f32,
            camera.pos.y * video_mode.height() as f32,
        );

        let camera_tile = Vec2::new(
            camera_pixel_pos.x / self.data.tile_width as f32,
            camera_pixel_pos.y / self.data.tile_height as f32,
        );

        let tiles_on_screen_x = (video_mode.width() / self.data.tile_width) + 1;
        let tiles_on_screen_y = (video_mode.height() / self.data.tile_height) + 2;

        let first_tile_x = camera_tile.x as i32;
        let first_tile_y = camera_tile.y as i32;

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

                    let upper_left = pos;
                    let lower_right = pos + tile_scale;

                    cb.set_pipeline(
                        &MAP_PIPELINE
                            .with_texture(Some(self.data.tiles[(tile - 1) as usize].as_texture())),
                    );

                    cb.add_textured_rect(
                        upper_left - camera_pixel_pos,
                        lower_right - camera_pixel_pos,
                    );
                }
            }
        }
    }

    pub fn get_start_pos(&self) -> Vec2 {
        Vec2::new(
            self.data.tile_width as f32,
            ((self.data.height_in_tiles - 1) * self.data.tile_height) as f32,
        )
    }
}
