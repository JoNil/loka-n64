use crate::bullet_system::BulletSystem;
use crate::components::box_drawable::{self, BoxDrawableComponent};
use crate::components::movable::{self, MovableComponent};
use crate::components::health::{self, HealthComponent};
use crate::entity::{self, OwnedEntity, Entity};
use n64::{current_time_us, graphics, ipl3font, Controllers};
use n64_math::{Color, Vec2};

const START_POS: Vec2 = Vec2::new(0.5, 0.8);
const SHIP_COLOR: Color = Color::new(0b10000_00011_00011_1);
const SHIP_SPEED: f32 = 0.35;
const SHIP_SHOOT_DELAY_MS: i32 = 150;
pub const SHIP_SIZE: Vec2 = Vec2::new(
    ipl3font::GLYPH_WIDTH as f32 / graphics::WIDTH as f32,
    ipl3font::GLYPH_HEIGHT as f32 / graphics::HEIGHT as f32,
);

pub struct Player {
    entity: OwnedEntity,
    score: i32,
    last_shoot_time: i64,
}

impl Player {
    pub fn new() -> Player {
        let player = Player {
            entity: entity::create(),
            score: 0,
            last_shoot_time: 0,
        };

        movable::add(&player.entity, MovableComponent {
            pos: START_POS,
            speed: Vec2::zero(),
        });
        box_drawable::add(&player.entity, BoxDrawableComponent {
            size: SHIP_SIZE,
            color: SHIP_COLOR,
        });
        health::add(&player.entity, HealthComponent {
            health: 500,
        });

        player
    }

    pub fn entity(&self) -> &Entity {
        &self.entity
    }

    pub fn add_score(&mut self, score: i32) {
        self.score += score;
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn update(
        &mut self,
        controllers: &Controllers,
        bullet_system: &mut BulletSystem,
    ) {
        let controller_x = controllers.x();
        let controller_y = controllers.y();

        let mut controller_dir = Vec2::new(0.0, 0.0);

        if controller_x.abs() > 32 {
            controller_dir.set_x(if controller_x > 0 { 1.0 } else { -1.0 });
        }

        if controller_y.abs() > 32 {
            controller_dir.set_y(if controller_y > 0 { -1.0 } else { 1.0 });
        }

        if let Some(movable) = movable::lock_mut().lookup_mut(&self.entity) {
            movable.speed = SHIP_SPEED * controller_dir;
        }

        if let Some(movable) = movable::get_component(&self.entity) {
            let now = current_time_us();

            if now - self.last_shoot_time > SHIP_SHOOT_DELAY_MS as i64 * 1000 {
                if controllers.z() {
                    bullet_system.shoot_bullet(
                        movable.pos + Vec2::new(0.0, -SHIP_SIZE.y() / 2.0),
                        Vec2::new(0.0, movable.speed.y() - 0.65),
                    );
                    self.last_shoot_time = now;
                }
            }
        }
    }
}
