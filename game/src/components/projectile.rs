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

    for (projectile, entity) in projectile.components_and_entities() {
        if let (Some(m), Some(s)) = (movable.lookup(entity), size.lookup(entity)) {
            let mut delete = false;
            let projectile_bb = Aabb2::from_center_size(m.pos, s.size);

            if !projectile_bb.collides(&camera_bb) {
                delete = true;
            }

            if projectile.target_type == WeaponTarget::Enemy {
                for enemy_entity in enemy.entities() {
                    if let Some(size) = size.lookup(*enemy_entity) {
                        let enemy_bb = Aabb2::from_center_size(
                            movable::pos(movable, *enemy_entity).unwrap_or(Vec2::ZERO),
                            size.size,
                        );

                        if projectile_bb.collides(&enemy_bb) {
                            health::damage(health, *enemy_entity, projectile.damage);
                            delete = true;
                        }
                    }
                }
            }

            if projectile.target_type == WeaponTarget::Player {
                for player_entity in player.entities() {
                    if let Some(s) = size.lookup(*player_entity) {
                        let player_bb = Aabb2::from_center_size(
                            movable::pos(movable, *player_entity).unwrap_or(Vec2::ZERO),
                            s.size,
                        );

                        if projectile_bb.collides(&player_bb) {
                            health::damage(health, *player_entity, projectile.damage);
                            delete = true;
                        }
                    }
                }
            }

            if delete || projectile.delete_after_first_frame {
                world.entities.despawn(entity);
            }
        }
    }
}
