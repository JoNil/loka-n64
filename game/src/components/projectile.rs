use super::{
    enemy::Enemy,
    health::{self, Health},
    movable::Movable,
    player::Player,
    size::Size,
    weapon::WeaponTarget,
};
use crate::{camera::Camera, ecs::world::World, sound_mixer::SoundMixer, sounds::HIT_1};
use n64_math::{vec2, Aabb2};

pub struct Projectile {
    pub target_type: WeaponTarget,
    pub damage: i32,
    pub projectile_collision_grace_period_ms: i32,
}

pub fn update(world: &mut World, sound_mixer: &mut SoundMixer, camera: &Camera, dt: f32) {
    let (projectile, movable, enemy, player, size, health) =
        world
            .components
            .get::<(Projectile, Movable, Enemy, Player, Size, Health)>();

    let camera_bb = Aabb2::new(camera.pos, camera.pos + vec2(1.0, 1.0));

    let projectiles = projectile.components();
    let entities = projectile.entities();

    for i in 0..projectiles.len() {
        let p1 = &projectiles[i];
        let e1 = entities[i];

        if let (Some(m), Some(s)) = (movable.lookup(e1), size.lookup(e1)) {
            let mut delete = false;
            let projectile_bb = Aabb2::from_center_size(m.pos, s.size);

            if !projectile_bb.collides(&camera_bb) {
                delete = true;
            }

            if p1.target_type == WeaponTarget::Enemy {
                for enemy_entity in enemy.entities() {
                    if let (Some(m2), Some(s2)) =
                        (movable.lookup(*enemy_entity), size.lookup(*enemy_entity))
                    {
                        let enemy_bb = Aabb2::from_center_size(m2.pos, s2.size);

                        if projectile_bb.collides(&enemy_bb) {
                            sound_mixer.play_sound(HIT_1.as_sound_data());
                            health::damage(health, *enemy_entity, p1.damage);
                            delete = true;
                        }
                    }
                }
            }

            if p1.target_type == WeaponTarget::Player {
                for player_entity in player.entities() {
                    if let (Some(m2), Some(s2)) =
                        (movable.lookup(*player_entity), size.lookup(*player_entity))
                    {
                        let player_bb = Aabb2::from_center_size(m2.pos, s2.size);

                        if projectile_bb.collides(&player_bb) {
                            sound_mixer.play_sound(HIT_1.as_sound_data());
                            health::damage(health, *player_entity, p1.damage);
                            delete = true;
                        }
                    }
                }
            }

            if p1.projectile_collision_grace_period_ms <= 0 {
                for j in (i + 1)..projectiles.len() {
                    let p2 = &projectiles[j];
                    let e2 = entities[j];

                    if p2.projectile_collision_grace_period_ms <= 0 {
                        if let (Some(m2), Some(s2)) = (movable.lookup(e2), size.lookup(e2)) {
                            let projectile_bb_2 = Aabb2::from_center_size(m2.pos, s2.size);

                            if projectile_bb.collides(&projectile_bb_2) {
                                sound_mixer.play_sound(HIT_1.as_sound_data());
                                health::damage(health, e2, p1.damage);
                                health::damage(health, e1, p2.damage);
                            }
                        }
                    }
                }
            }

            if !health::is_alive(health, e1) {
                delete = true;
            }

            if delete {
                world.entities.despawn(e1);
            }
        }
    }

    for p in projectile.components_mut() {
        p.projectile_collision_grace_period_ms =
            (p.projectile_collision_grace_period_ms - (dt * 1000.0) as i32).max(0);
    }
}
