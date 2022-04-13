use super::{
    box_drawable::BoxDrawable,
    enemy::Enemy,
    health::{self, Health},
    movable::{self, Movable},
    player::Player,
    size::Size,
    weapon::WeaponTarget,
};
use crate::{
    camera::Camera,
    ecs::{
        entity::{Entity, EntitySystem},
        world::World,
    },
};
use n64_math::{const_vec2, vec2, Aabb2, Color, Mat2, Vec2};

const MISSILE_SIZE: Vec2 = const_vec2!([4.0 * 0.00825, 4.0 * 0.00825]);

struct Missile {
    pub target: Option<Entity>,
    pub target_type: WeaponTarget,
}

pub fn shoot_missile(
    entities: &mut EntitySystem,
    pos: Vec2,
    offset: Vec2,
    speed: Vec2,
    speed_offset: Vec2,
    direction: f32,
    target: Option<Entity>,
    target_type: WeaponTarget,
) {
    let spread = (n64_math::random_f32() - 0.5) * 0.05;

    let rot = Mat2::from_angle(direction);

    let offset = rot.mul_vec2(offset);
    let speed_offset =
        Mat2::from_angle(direction).mul_vec2(vec2(speed_offset.x + spread, speed_offset.y));

    entities
        .spawn()
        .add(Movable {
            pos: pos + offset,
            speed: speed + speed_offset,
        })
        .add(Size { size: MISSILE_SIZE })
        .add(BoxDrawable {
            color: Color::from_rgb(1.0, 0.4, 0.4),
        })
        .add(Missile {
            target,
            target_type,
        });
}

pub fn update(world: &mut World, camera: &Camera) {
    let (missile, movable, enemy, player, size, health) = world
        .components
        .get6::<Missile, Movable, Enemy, Player, Size, Health>();

    let camera_bb: Aabb2 = Aabb2::new(camera.pos, camera.pos + Vec2::new(1.0, 1.0));

    for (missile, entity) in missile.components_and_entities() {
        let target_pos = missile
            .target
            .and_then(|target| movable::pos(movable, target));

        if let Some(m) = movable.lookup_mut(entity) {
            if let Some(target_pos) = target_pos {
                let towords_target = (target_pos - m.pos).normalize();
                let speed_dir = m.speed.normalize();
                let new_speed_dir = (0.05 * towords_target + 0.95 * speed_dir).normalize();
                let new_speed = new_speed_dir * m.speed.length();
                m.speed = new_speed;
            }

            let mut delete = false;
            let missile_bb = Aabb2::from_center_size(m.pos, MISSILE_SIZE);

            if !missile_bb.collides(&camera_bb) {
                delete = true;
            }

            if missile.target_type == WeaponTarget::Enemy {
                for enemy_entity in enemy.entities() {
                    if let Some(size) = size.lookup(*enemy_entity) {
                        let enemy_bb = Aabb2::from_center_size(
                            movable::pos(movable, *enemy_entity).unwrap_or(Vec2::ZERO),
                            size.size,
                        );

                        if missile_bb.collides(&enemy_bb) {
                            health::damage(
                                health,
                                *enemy_entity,
                                100 + (n64_math::random_f32() * 50.0) as i32,
                            );
                            delete = true;
                        }
                    }
                }
            }

            if missile.target_type == WeaponTarget::Player {
                for player_entity in player.entities() {
                    if let Some(size) = size.lookup(*player_entity) {
                        let player_bb = Aabb2::from_center_size(
                            movable::pos(movable, *player_entity).unwrap_or(Vec2::ZERO),
                            size.size,
                        );

                        if missile_bb.collides(&player_bb) {
                            health::damage(
                                health,
                                *player_entity,
                                100 + (n64_math::random_f32() * 50.0) as i32,
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
