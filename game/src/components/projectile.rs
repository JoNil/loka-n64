use super::{
    enemy::Enemy,
    health::{self, Health},
    movable::Movable,
    player::Player,
    size::Size,
    weapon::WeaponTarget,
};
use crate::{camera::Camera, ecs::world::World};
use n64_math::{vec2, Aabb2};

pub struct Projectile {
    pub target_type: WeaponTarget,
    pub damage: i32,
}

pub fn update(world: &mut World, camera: &Camera) {
    let (projectile, movable, enemy, player, size, health) = world
        .components
        .get6::<Projectile, Movable, Enemy, Player, Size, Health>();

    let camera_bb: Aabb2 = Aabb2::new(camera.pos, camera.pos + vec2(1.0, 1.0));

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
                            health::damage(health, *player_entity, p1.damage);
                            delete = true;
                        }
                    }
                }
            }

            for j in (i + 1)..projectiles.len() {
                let p2 = &projectiles[j];
                let e2 = entities[j];

                if let (Some(m2), Some(s2)) = (movable.lookup(e2), size.lookup(e2)) {
                    let projectile_bb_2 = Aabb2::from_center_size(m2.pos, s2.size);

                    if projectile_bb.collides(&projectile_bb_2) {
                        health::damage(health, e2, p1.damage);
                        health::damage(health, e1, p2.damage);
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
}