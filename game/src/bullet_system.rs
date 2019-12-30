use alloc::vec::Vec;
use crate::{Player, SHIP_SIZE};
use crate::enemy_system::{EnemySystem, ENEMY_SIZE};
use crate::entity::{OwnedEntity, self};
use crate::components::movable::{self, MovableComponent};
use crate::components::char_drawable::{self, CharDrawableComponent};
use n64_math::{Aabb2, Color, Vec2};
use n64::Rng;

const BULLET_SIZE: Vec2 = Vec2::new(2.0 / 320.0, 2.0 / 320.0);

struct Bullet {
    entity: OwnedEntity,
    can_hit_player: bool,
    can_hit_enemy: bool,
}

pub struct BulletSystem {
    bullets: Vec<Bullet>,
    screen_bb: Aabb2,
}

impl BulletSystem {
    pub fn new() -> BulletSystem {
        BulletSystem {
            bullets: Vec::new(),
            screen_bb: Aabb2::new(Vec2::zero(), Vec2::new(1.0, 1.0)),
        }
    }

    pub fn shoot_bullet(&mut self, rng: &mut Rng, pos: Vec2, speed: Vec2) {

        let entity = entity::create();
        movable::add(MovableComponent { entity: entity.as_entity(), pos: pos, speed: speed });
        char_drawable::add(CharDrawableComponent {
            entity: entity.as_entity(), 
            color: Color::from_rgb(rng.next_f32(), rng.next_f32(), rng.next_f32()),
            chr: '.',
        });

        self.bullets.push(Bullet {
            entity: entity,
            can_hit_player: false,
            can_hit_enemy: true,
        });
    }

    pub fn shoot_bullet_enemy(&mut self, rng: &mut Rng, pos: Vec2, speed: Vec2) {

        let entity = entity::create();
        movable::add(MovableComponent { entity: entity.as_entity(), pos: pos, speed: speed });
        char_drawable::add(CharDrawableComponent {
            entity: entity.as_entity(), 
            color: Color::from_rgb(rng.next_f32(), rng.next_f32(), rng.next_f32()),
            chr: '.',
        });

        self.bullets.push(Bullet {
            entity: entity,
            can_hit_player: true,
            can_hit_enemy: false,
        });
    }

    pub fn update(&mut self, enemy_system: &mut EnemySystem, player: &mut Player, rng: &mut Rng) {
        let mut delete_list = Vec::new();

        for (i, bullet) in self.bullets.iter_mut().enumerate() {

            if let Some(movable) = movable::get_component(&bullet.entity) {

                let bullet_bb = Aabb2::from_center_size(movable.pos, BULLET_SIZE);

                if !bullet_bb.collides(&self.screen_bb) {
                    delete_list.push(i);
                }

                if bullet.can_hit_enemy {
                    for enemy in enemy_system.enemies_mut() {
                        let enemy_bb = Aabb2::from_center_size(enemy.pos(), ENEMY_SIZE);

                        if bullet_bb.collides(&enemy_bb) {
                            enemy.damage(50 + (rng.next_f32() * 20.0) as i32);
                            delete_list.push(i);
                        }
                    }
                }

                if bullet.can_hit_player {
                    let player_bb = Aabb2::from_center_size(player.pos(), SHIP_SIZE);

                    if bullet_bb.collides(&player_bb) {
                        player.damage(50 + (rng.next_f32() * 20.0) as i32);
                        delete_list.push(i);
                    }
                }
            }
        }

        {
            let len = self.bullets.len();

            for (i, delete_index) in delete_list.iter().enumerate() {    
                self.bullets.swap(*delete_index, len - 1 - i);
            }

            self.bullets.drain((len - delete_list.len())..);
        }
    }
}
