use super::{
    bullet::shoot_bullet,
    enemy::Enemy,
    health::Health,
    missile::shoot_missile,
    movable::{self, Movable},
    sprite_drawable::SpriteDrawable,
};
use crate::{
    camera::Camera,
    entity::Entity,
    sound_mixer::SoundMixer,
    sounds::{SHOOT_1, SHOOT_2},
    textures::SHIP_2_SMALL,
    weapon::Weapon,
    world::World,
};
use n64::{current_time_us, Controllers};
use n64_math::Vec2;

const PLAYTER_START_POS: Vec2 = Vec2::new(0.5, 0.8);
const SHIP_SPEED: f32 = 0.35;
const SHIP_SHOOT_DELAY_MS: i32 = 150;
const SHIP_SHOOT_MISSILE_DELAY_MS: i32 = 1000;
pub const SHIP_SIZE: Vec2 = Vec2::new(32.0 / 320.0, 32.0 / 240.0);

#[derive(Copy, Clone)]
pub struct Player {
    pub score: i32,
    pub last_shoot_time: i64,
    pub weapon: Weapon,
}

pub fn spawn_player(world: &mut World, start_pos: Vec2) -> Entity {
    let entity = world.entities.create();
    world.add(
        entity,
        Movable {
            pos: start_pos + PLAYTER_START_POS,
            speed: Vec2::new(0.0, 0.0),
        },
    );
    world.add(
        entity,
        SpriteDrawable {
            size: SHIP_SIZE,
            texture: SHIP_2_SMALL.as_texture(),
        },
    );
    world.add(entity, Health { health: 10000 });
    world.add(
        entity,
        Player {
            score: 0,
            last_shoot_time: 0,
            weapon: Weapon::Missile,
        },
    );
    entity
}

pub fn add_score(world: &mut World, score: i32) {
    for mut player in world.components_mut::<Player>() {
        player.score += score;
    }
}

pub fn update(
    world: &mut World,
    controllers: &Controllers,
    sound_mixer: &mut SoundMixer,
    camera: &Camera,
) {
    for (player, entity) in world.components_and_entities_mut::<Player>() {
        let controller_x = controllers.x();
        let controller_y = controllers.y();

        let mut controller_dir = Vec2::new(0.0, 0.0);

        if controller_x.abs() > 32 {
            controller_dir.set_x(if controller_x > 0 { 1.0 } else { -1.0 });
        }

        if controller_y.abs() > 32 {
            controller_dir.set_y(if controller_y > 0 { -1.0 } else { 1.0 });
        }

        if let Some(movable) = world.lookup_mut::<Movable>(entity) {
            movable.speed = SHIP_SPEED * controller_dir - camera.speed;
        }

        if let Some(movable) = world.lookup::<Movable>(entity).copied() {
            let now = current_time_us();

            match player.weapon {
                Weapon::Bullet => {
                    if now - player.last_shoot_time > SHIP_SHOOT_DELAY_MS as i64 * 1000
                        && controllers.z()
                    {
                        sound_mixer.play_sound(SHOOT_1.as_sound_data());
                        shoot_bullet(
                            world,
                            movable.pos + Vec2::new(0.0, -SHIP_SIZE.y() / 2.0),
                            Vec2::new(0.0, movable.speed.y() - 1.25),
                        );
                        player.last_shoot_time = now;
                    }
                }
                Weapon::Missile => {
                    if now - player.last_shoot_time > SHIP_SHOOT_MISSILE_DELAY_MS as i64 * 1000
                        && controllers.z()
                    {
                        sound_mixer.play_sound(SHOOT_2.as_sound_data());

                        let player_pos = movable.pos;

                        let mut distances = world
                            .entities::<Enemy>()
                            .iter()
                            .filter_map(|e| movable::pos(world, *e).map(|pos| (pos, e)))
                            .map(|(pos, e)| ((player_pos - pos).length(), *e))
                            .collect::<Vec<_>>();

                        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

                        let target_1 = distances.get(0).map(|(_, e)| *e);
                        let target_2 = distances.get(1).map(|(_, e)| *e);
                        let target_3 = distances.get(2).map(|(_, e)| *e);

                        shoot_missile(
                            world,
                            movable.pos + Vec2::new(0.0, -SHIP_SIZE.y() / 2.0),
                            Vec2::new(0.0, movable.speed.y() - 0.5),
                            target_1,
                        );
                        shoot_missile(
                            world,
                            movable.pos + Vec2::new(0.0, -SHIP_SIZE.y() / 2.0),
                            Vec2::new(0.15, movable.speed.y() - 0.5),
                            target_2,
                        );
                        shoot_missile(
                            world,
                            movable.pos + Vec2::new(0.0, -SHIP_SIZE.y() / 2.0),
                            Vec2::new(-0.15, movable.speed.y() - 0.5),
                            target_3,
                        );
                        player.last_shoot_time = now;
                    }
                }
            }
        }
    }
}
