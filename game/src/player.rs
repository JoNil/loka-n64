use crate::bullet_system::BulletSystem;
use crate::components::health::HealthComponent;
use crate::components::movable::{self, MovableComponent};
use crate::components::sprite_drawable::SpriteDrawableComponent;
use crate::enemy_system::{self, EnemySystem};
use crate::entity::{Entity, OwnedEntity};
use crate::missile_system::{self, MissileSystem};
use crate::weapon::Weapon;
use crate::{
    camera::Camera,
    sound_mixer::SoundMixer,
    sounds::{SHOOT_1, SHOOT_2},
    textures::SHIP_2_SMALL,
    world::World,
};
use alloc::vec::Vec;
use n64::{current_time_us, Controllers};
use n64_math::Vec2;

const PLAYTER_START_POS: Vec2 = Vec2::new(0.5, 0.8);
const SHIP_SPEED: f32 = 0.35;
const SHIP_SHOOT_DELAY_MS: i32 = 150;
const SHIP_SHOOT_MISSILE_DELAY_MS: i32 = 1000;
pub const SHIP_SIZE: Vec2 = Vec2::new(32.0 / 320.0, 32.0 / 240.0);

pub struct Player {
    entity: OwnedEntity,
    score: i32,
    last_shoot_time: i64,
    weapon: Weapon,
}

impl Player {
    pub fn new(world: &mut World, start_pos: Vec2) -> Self {
        let player = Player {
            entity: world.entity.create(),
            score: 0,
            last_shoot_time: 0,
            weapon: Weapon::Missile,
        };

        world.movable.add(
            &player.entity,
            MovableComponent {
                pos: start_pos + PLAYTER_START_POS,
                speed: Vec2::new(0.0, 0.0),
            },
        );
        world.sprite_drawable.add(
            &player.entity,
            SpriteDrawableComponent {
                size: SHIP_SIZE,
                texture: SHIP_2_SMALL.as_texture(),
            },
        );
        world
            .health
            .add(&player.entity, HealthComponent { health: 10000 });

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

    pub fn update(
        &mut self,
        world: &mut World,
        controllers: &Controllers,
        bullet_system: &mut BulletSystem,
        missile_system: &mut MissileSystem,
        enemy_system: &EnemySystem,
        sound_mixer: &mut SoundMixer,
        camera: &Camera,
    ) {
        let controller_x = controllers.x();
        let controller_y = controllers.y();

        let mut controller_dir = Vec2::new(0.0, 0.0);

        if controller_x.abs() > 32 {
            controller_dir.set_x(if controller_x > 0 { 1.0 } else { -1.0 });
        }

        if controller_y.abs() > 32 {
            controller_dir.set_y(if controller_y > 0 { -1.0 } else { 1.0 });
        }

        if let Some(movable) = world.movable.lookup_mut(&self.entity) {
            movable.speed = SHIP_SPEED * controller_dir - camera.speed;
        }

        if let Some(movable) = world.movable.lookup(&self.entity).copied() {
            let now = current_time_us();

            match self.weapon {
                Weapon::Bullet => {
                    if now - self.last_shoot_time > SHIP_SHOOT_DELAY_MS as i64 * 1000
                        && controllers.z()
                    {
                        sound_mixer.play_sound(SHOOT_1.as_sound_data());
                        bullet_system.shoot_bullet(
                            world,
                            movable.pos + Vec2::new(0.0, -SHIP_SIZE.y() / 2.0),
                            Vec2::new(0.0, movable.speed.y() - 1.25),
                        );
                        self.last_shoot_time = now;
                    }
                }
                Weapon::Missile => {
                    if now - self.last_shoot_time > SHIP_SHOOT_MISSILE_DELAY_MS as i64 * 1000
                        && controllers.z()
                    {
                        sound_mixer.play_sound(SHOOT_2.as_sound_data());

                        let player_pos = movable.pos;

                        let mut distances = enemy_system
                            .enemies()
                            .iter()
                            .filter_map(|e| {
                                if let Some(pos) = world.movable.pos(e.entity()) {
                                    Some((pos, e))
                                } else {
                                    None
                                }
                            })
                            .map(|(pos, e)| ((player_pos - pos).length(), e))
                            .collect::<Vec<_>>();

                        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

                        let target_1 = distances.get(0).map(|(d, e)| *e);
                        let target_2 = distances.get(1).map(|(d, e)| *e);
                        let target_3 = distances.get(2).map(|(d, e)| *e);

                        missile_system.shoot_missile(
                            world,
                            movable.pos + Vec2::new(0.0, -SHIP_SIZE.y() / 2.0),
                            Vec2::new(0.0, movable.speed.y() - 0.5),
                            target_1.map(|e| *e.entity()),
                        );
                        missile_system.shoot_missile(
                            world,
                            movable.pos + Vec2::new(0.0, -SHIP_SIZE.y() / 2.0),
                            Vec2::new(0.15, movable.speed.y() - 0.5),
                            target_2.map(|e| *e.entity()),
                        );
                        missile_system.shoot_missile(
                            world,
                            movable.pos + Vec2::new(0.0, -SHIP_SIZE.y() / 2.0),
                            Vec2::new(-0.15, movable.speed.y() - 0.5),
                            target_3.map(|e| *e.entity()),
                        );
                        self.last_shoot_time = now;
                    }
                }
            }
        }
    }
}
