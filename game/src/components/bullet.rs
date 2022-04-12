use super::{
    box_drawable::BoxDrawable,
    enemy::Enemy,
    health::{self, Health},
    movable::{self, Movable},
    player::Player,
    size::Size,
};
use crate::{
    camera::Camera,
    ecs::{entity::EntitySystem, world::World},
};
use n64_math::{const_vec2, vec2, Aabb2, Color, Vec2};

const BULLET_SIZE: Vec2 = const_vec2!([0.00825, 0.00825]);

struct Bullet {
    pub can_hit_player: bool,
    pub can_hit_enemy: bool,
}

pub fn shoot_bullet(entities: &mut EntitySystem, pos: Vec2, speed: Vec2) {
    let spread = (n64_math::random_f32() - 0.5) * 0.05;

    entities
        .spawn()
        .add(Movable {
            pos,
            speed: vec2(speed.x + spread, speed.y),
        })
        .add(Size { size: BULLET_SIZE })
        .add(BoxDrawable {
            color: Color::from_rgb(0.2, 0.2, 0.9),
        })
        .add(Bullet {
            can_hit_player: false,
            can_hit_enemy: true,
        });
}

pub fn shoot_bullet_enemy(entities: &mut EntitySystem, pos: Vec2, speed: Vec2) {
    entities
        .spawn()
        .add(Movable { pos, speed })
        .add(Size { size: BULLET_SIZE })
        .add(BoxDrawable {
            color: Color::from_rgb(0.9, 0.2, 0.2),
        })
        .add(Bullet {
            can_hit_player: true,
            can_hit_enemy: false,
        });
}

pub fn update(world: &mut World, camera: &Camera) {
    let (bullet, movable, enemy, player, size, health) = world
        .components
        .get6::<Bullet, Movable, Enemy, Player, Size, Health>();

    let camera_bb: Aabb2 = Aabb2::new(camera.pos, camera.pos + vec2(1.0, 1.0));

    for (bullet, entity) in bullet.components_and_entities() {
        if let Some(m) = movable.lookup(entity) {
            let mut delete = false;
            let bullet_bb = Aabb2::from_center_size(m.pos, BULLET_SIZE);

            if !bullet_bb.collides(&camera_bb) {
                delete = true;
            }

            if bullet.can_hit_enemy {
                for enemy_entity in enemy.entities() {
                    if let Some(size) = size.lookup(*enemy_entity) {
                        let enemy_bb = Aabb2::from_center_size(
                            movable::pos(movable, *enemy_entity).unwrap_or(Vec2::ZERO),
                            size.size,
                        );

                        if bullet_bb.collides(&enemy_bb) {
                            health::damage(
                                health,
                                *enemy_entity,
                                50 + (n64_math::random_f32() * 20.0) as i32,
                            );
                            delete = true;
                        }
                    }
                }
            }

            if bullet.can_hit_player {
                for player_entity in player.entities() {
                    if let Some(s) = size.lookup(*player_entity) {
                        let player_bb = Aabb2::from_center_size(
                            movable::pos(movable, *player_entity).unwrap_or(Vec2::ZERO),
                            s.size,
                        );

                        if bullet_bb.collides(&player_bb) {
                            health::damage(
                                health,
                                *player_entity,
                                50 + (n64_math::random_f32() * 20.0) as i32,
                            );
                            delete = true;
                        }
                    }
                }
            }

            if delete {
                world.entities.despawn(entity);
            }
        }
    }
}
