use super::{
    enemy::Enemy,
    health::{self, Health},
    mesh_drawable::MeshDrawable,
    movable::{self, Movable},
    player::Player,
    size::Size,
    weapon::WeaponTarget,
};
use crate::{
    ecs::{entity::EntitySystem, world::World},
    models::LASER_BODY,
};
use n64_math::{vec2, Aabb2, Quat, Vec2};

struct Laser {
    target: WeaponTarget,
}

pub fn shoot_laser(entities: &mut EntitySystem, pos: Vec2, speed: Vec2, target: WeaponTarget) {
    entities
        .spawn()
        .add(Movable {
            pos: pos + vec2(0.0, -LASER_BODY.size.y / 2.0),
            speed,
        })
        .add(Size {
            size: LASER_BODY.size,
        })
        .add(MeshDrawable {
            model: LASER_BODY.as_model_data(),
            rot: Quat::IDENTITY,
        })
        .add(Laser { target });
}

pub fn update(world: &mut World) {
    let (laser, movable, enemy, player, size, health) = world
        .components
        .get6::<Laser, Movable, Enemy, Player, Size, Health>();

    for (laser, entity) in laser.components_and_entities() {
        if let Some(m) = movable.lookup(entity) {
            let laser_bb = Aabb2::from_center_size(m.pos, LASER_BODY.size);

            if laser.target == WeaponTarget::Enemy {
                for enemy_entity in enemy.entities() {
                    if let Some(size) = size.lookup(*enemy_entity) {
                        let enemy_bb = Aabb2::from_center_size(
                            movable::pos(movable, *enemy_entity).unwrap_or(Vec2::ZERO),
                            size.size,
                        );

                        if laser_bb.collides(&enemy_bb) {
                            health::damage(health, *enemy_entity, 1 as i32);
                        }
                    }
                }
            }

            if laser.target == WeaponTarget::Player {
                for player_entity in player.entities() {
                    if let Some(s) = size.lookup(*player_entity) {
                        let player_bb = Aabb2::from_center_size(
                            movable::pos(movable, *player_entity).unwrap_or(Vec2::ZERO),
                            s.size,
                        );

                        if laser_bb.collides(&player_bb) {
                            health::damage(health, *player_entity, 1);
                        }
                    }
                }
            }

            world.entities.despawn(entity);
        }
    }
}
