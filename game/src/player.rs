use crate::bullet_system::BulletSystem;
use crate::components::health::{self, HealthComponent};
use crate::components::movable::{self, MovableComponent};
use crate::components::sprite_drawable::{self, SpriteDrawableComponent};
use crate::entity::{self, Entity, OwnedEntity};
use crate::{sound_mixer::SoundMixer, textures::SHIP_2_SMALL, sounds::SHOOT_1};
use n64::{current_time_us, Controllers};
use n64_math::Vec2;

const START_POS: Vec2 = Vec2::new(0.5, 0.8);
const SHIP_SPEED: f32 = 0.35;
const SHIP_SHOOT_DELAY_MS: i32 = 150;
pub const SHIP_SIZE: Vec2 = Vec2::new(32.0 / 320.0 as f32, 32.0 / 240.0 as f32);

pub struct Player {
    entity: OwnedEntity,
    score: i32,
    last_shoot_time: i64,
}

impl Player {
    pub fn new() -> Self {
        let player = Player {
            entity: entity::create(),
            score: 0,
            last_shoot_time: 0,
        };

        movable::add(
            &player.entity,
            MovableComponent {
                pos: START_POS,
                speed: Vec2::zero(),
            },
        );
        sprite_drawable::add(
            &player.entity,
            SpriteDrawableComponent {
                size: SHIP_SIZE,
                texture: SHIP_2_SMALL.as_texture(),
            },
        );
        health::add(&player.entity, HealthComponent { health: 5000 });

        player
    }

    pub fn entity(&self) -> &Entity {
        &self.entity
    }

    pub fn add_score(&mut self, score: i32) {
        self.score += score;
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn update(&mut self, controllers: &Controllers, bullet_system: &mut BulletSystem, sound_mixer: &mut SoundMixer) {
        let controller_x = controllers.x();
        let controller_y = controllers.y();

        let mut controller_dir = Vec2::new(0.0, 0.0);

        if controller_x.abs() > 32 {
            controller_dir.set_x(if controller_x > 0 { 1.0 } else { -1.0 });
        }

        if controller_y.abs() > 32 {
            controller_dir.set_y(if controller_y > 0 { -1.0 } else { 1.0 });
        }

        if let Some(movable) = movable::lock_mut().lookup_mut(&self.entity) {
            movable.speed = SHIP_SPEED * controller_dir;
        }

        if let Some(movable) = movable::get_component(&self.entity) {
            let now = current_time_us();

            if now - self.last_shoot_time > SHIP_SHOOT_DELAY_MS as i64 * 1000 {
                if controllers.z() {
                    sound_mixer.play_sound(SHOOT_1.as_sound_data());
                    bullet_system.shoot_bullet(
                        movable.pos + Vec2::new(0.0, -SHIP_SIZE.y() / 2.0),
                        Vec2::new(0.0, movable.speed.y() - 0.65),
                    );
                    self.last_shoot_time = now;
                }
            }
        }
    }
}
