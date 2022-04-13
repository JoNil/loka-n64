use super::{
    enemy::Enemy,
    health::{self, Health},
    movable::{self, Movable},
    player::Player,
    size::Size,
    weapon::WeaponTarget,
};
use crate::{camera::Camera, ecs::world::World};
use n64_math::{vec2, Aabb2, Vec2};

pub struct Projectile {
    pub target_type: WeaponTarget,
    pub damage: i32,
    pub delete_after_first_frame: bool,
}

pub fn update(world: &mut World, camera: &Camera) {
    let (projectile, movable, enemy, player, size, health) = world
        .components
        .get6::<Projectile, Movable, Enemy, Player, Size, Health>();

    let camera_bb: Aabb2 = Aabb2::new(camera.pos, camera.pos + vec2(1.0, 1.0));

    for (p, entity) in projectile.components_and_entities() {
        if let (Some(m), Some(s)) = (movable.lookup(entity), size.lookup(entity)) {
            let mut delete = false;
            let projectile_bb = Aabb2::from_center_size(m.pos, s.size);

            if !projectile_bb.collides(&camera_bb) {
                delete = true;
            }

            if p.target_type == WeaponTarget::Enemy {
                for enemy_entity in enemy.entities() {
                    if let (Some(m2), Some(s2)) =
                        (movable.lookup(*enemy_entity), size.lookup(*enemy_entity))
                    {
                        let enemy_bb = Aabb2::from_center_size(m2.pos, s2.size);

                        if projectile_bb.collides(&enemy_bb) {
                            health::damage(health, *enemy_entity, p.damage);
                            delete = true;
                        }
                    }
                }
            }

            if p.target_type == WeaponTarget::Player {
                for player_entity in player.entities() {
                    if let (Some(m2), Some(s2)) =
                        (movable.lookup(*player_entity), size.lookup(*player_entity))
                    {
                        let player_bb = Aabb2::from_center_size(m2.pos, s2.size);

                        if projectile_bb.collides(&player_bb) {
                            health::damage(health, *player_entity, p.damage);
                            delete = true;
                        }
                    }
                }
            }

            for (p2, entity2) in projectile.components_and_entities() {
                if entity != entity2 {
                    if let (Some(m2), Some(s2)) = (movable.lookup(entity2), size.lookup(entity2)) {
                        let projectile_bb_2 = Aabb2::from_center_size(m2.pos, s2.size);

                        if projectile_bb.collides(&projectile_bb_2) {
                            health::damage(health, entity2, p.damage);
                            health::damage(health, entity, p2.damage);
                        }
                    }
                }
            }

            if !health::is_alive(health, entity) {
                delete = true;
            }

            if delete || p.delete_after_first_frame {
                world.entities.despawn(entity);
            }
        }
    }
}
