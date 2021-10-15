use crate::components::box_drawable::BoxDrawableComponent;
use crate::components::movable::{self, MovableComponent};
use crate::enemy_system::EnemySystem;
use crate::entity::{Entity, OwnedEntity};
use crate::{camera::Camera, world::World, Player, SHIP_SIZE};
use alloc::vec::Vec;
use n64_math::{self, Aabb2, Color, Vec2};

const MISSILE_SIZE: Vec2 = Vec2::new(4.0 * 0.00825, 4.0 * 0.00825);

struct Missile {
    entity: OwnedEntity,
    target: Option<Entity>,
}

pub struct MissileSystem {
    missiles: Vec<Missile>,
}

impl MissileSystem {
    pub fn new() -> Self {
        Self {
            missiles: Vec::new(),
        }
    }

    pub fn shoot_missile(
        &mut self,
        world: &mut World,
        pos: Vec2,
        speed: Vec2,
        target: Option<Entity>,
    ) {
        let spread = (n64_math::random_f32() - 0.5) * 0.05;

        let entity = world.entity.create();
        world.movable.add(
            &entity,
            MovableComponent {
                pos,
                speed: Vec2::new(speed.x() + spread, speed.y()),
            },
        );
        world.box_drawable.add(
            &entity,
            BoxDrawableComponent {
                size: MISSILE_SIZE,
                color: Color::from_rgb(1.0, 0.4, 0.4),
            },
        );

        self.missiles.push(Missile { entity, target });
    }

    pub fn update(
        &mut self,
        world: &mut World,
        enemy_system: &mut EnemySystem,
        player: &mut Player,
        camera: &Camera,
    ) {
        let mut delete_list = Vec::new();

        let camera_bb: Aabb2 = Aabb2::new(camera.pos, camera.pos + Vec2::new(1.0, 1.0));

        for (i, missile) in self.missiles.iter_mut().enumerate() {
            let target_pos = missile.target.and_then(|target| world.movable.pos(&target));

            if let Some(movable) = world.movable.lookup_mut(&missile.entity) {
                if let Some(target_pos) = target_pos {
                    let towords_target = (target_pos - movable.pos).normalize();
                    let speed_dir = movable.speed.normalize();
                    let new_speed_dir = (0.2 * towords_target + 0.8 * speed_dir).normalize();
                    let new_speed = new_speed_dir * movable.speed.length();
                    movable.speed = new_speed;
                }

                let mut delete = false;
                let missile_bb = Aabb2::from_center_size(movable.pos, MISSILE_SIZE);

                if !missile_bb.collides(&camera_bb) {
                    delete = true;
                }

                for enemy in enemy_system.enemies_mut() {
                    if let Some(sprite_drawable) = world.sprite_drawable.lookup(enemy.entity()) {
                        let enemy_bb = Aabb2::from_center_size(
                            world.movable.pos(enemy.entity()).unwrap_or_else(Vec2::zero),
                            sprite_drawable.size,
                        );

                        if missile_bb.collides(&enemy_bb) {
                            world.health.damage(
                                enemy.entity(),
                                100 + (n64_math::random_f32() * 50.0) as i32,
                            );
                            delete = true;
                        }
                    }
                }

                if delete {
                    delete_list.push(i);
                }
            }
        }

        {
            let len = self.missiles.len();

            for (i, delete_index) in delete_list.iter().enumerate() {
                self.missiles.swap(*delete_index, len - 1 - i);
            }

            self.missiles.drain((len - delete_list.len())..);
        }
    }
}
