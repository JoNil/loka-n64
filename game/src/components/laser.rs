use super::{
    enemy::Enemy,
    health::{self, Health},
    mesh_drawable::MeshDrawable,
    movable::{self, Movable},
    player::Player,
    size::Size,
};
use crate::{
    camera::Camera,
    ecs::{entity::EntitySystem, world::World},
    models::LASER_BODY,
};
use n64_math::{const_vec2, vec2, Aabb2, Quat, Vec2};

const LASER_SIZE: Vec2 = const_vec2!([0.00825, 10.0 * 0.00825]);

struct Laser {
    pub can_hit_player: bool,
    pub can_hit_enemy: bool,
}

pub fn shoot_laser(entities: &mut EntitySystem, pos: Vec2) {
    entities
        .spawn()
        .add(Movable {
            pos,
            speed: vec2(0.0, 0.0),
        })
        .add(Size { size: LASER_SIZE })
        .add(MeshDrawable {
            model: LASER_BODY.as_model_data(),
            rot: Quat::IDENTITY,
            scale: 1.0 / 55.0,
        })
        .add(Laser {
            can_hit_player: false,
            can_hit_enemy: true,
        });
}

pub fn update(world: &mut World, camera: &Camera) {
    let (laser, movable, enemy, player, size, health) = world
        .components
        .get6::<Laser, Movable, Enemy, Player, Size, Health>();

    for (laser, entity) in laser.components_and_entities() {
        if let Some(m) = movable.lookup(entity) {
            let laser_bb = Aabb2::from_center_size(m.pos, LASER_SIZE);

            if laser.can_hit_enemy {
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

            if laser.can_hit_player {
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
