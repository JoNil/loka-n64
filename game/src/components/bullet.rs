use super::{
    box_drawable::BoxDrawable,
    enemy::Enemy,
    health::{self, Health},
    movable::{self, Movable},
    player::{Player, SHIP_SIZE},
    sprite_drawable::SpriteDrawable,
};
use crate::{
    camera::Camera,
    ecs::{entity::EntitySystem, world::World},
};
use n64_math::{Aabb2, Color, Vec2};

const BULLET_SIZE: Vec2 = Vec2::new(0.00825, 0.00825);

#[derive(Copy, Clone)]
struct Bullet {
    can_hit_player: bool,
    can_hit_enemy: bool,
}

pub fn shoot_bullet(entities: &mut EntitySystem, pos: Vec2, speed: Vec2) {
    let spread = (n64_math::random_f32() - 0.5) * 0.05;

    entities
        .spawn()
        .add(Movable {
            pos,
            speed: Vec2::new(speed.x() + spread, speed.y()),
        })
        .add(BoxDrawable {
            size: BULLET_SIZE,
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
        .add(BoxDrawable {
            size: BULLET_SIZE,
            color: Color::from_rgb(0.9, 0.2, 0.2),
        })
        .add(Bullet {
            can_hit_player: true,
            can_hit_enemy: false,
        });
}

pub fn update(world: &mut World, camera: &Camera) {
    let (bullet, movable, enemy, player, sprite_drawable, health) = world
        .components
        .get6::<Bullet, Movable, Enemy, Player, SpriteDrawable, Health>();

    let camera_bb: Aabb2 = Aabb2::new(camera.pos, camera.pos + Vec2::new(1.0, 1.0));

    for (bullet, entity) in bullet.components_and_entities() {
        if let Some(m) = movable.lookup(entity) {
            let mut delete = false;
            let bullet_bb = Aabb2::from_center_size(m.pos, BULLET_SIZE);

            if !bullet_bb.collides(&camera_bb) {
                delete = true;
            }

            if bullet.can_hit_enemy {
                for enemy_entity in enemy.entities() {
                    if let Some(sprite_drawable) = sprite_drawable.lookup(*enemy_entity) {
                        let enemy_bb = Aabb2::from_center_size(
                            movable::pos(&movable, *enemy_entity).unwrap_or_else(Vec2::zero),
                            sprite_drawable.size,
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
                    let player_bb = Aabb2::from_center_size(
                        movable::pos(&movable, *player_entity).unwrap_or_else(Vec2::zero),
                        SHIP_SIZE,
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

            if delete {
                entity.despawn();
            }
        }
    }
}
