use crate::bullet_system::BulletSystem;
use crate::components::char_drawable::{self, CharDrawableComponent};
use crate::components::movable::{self, MovableComponent};
use crate::entity::{self, OwnedEntity};
use n64::{current_time_us, graphics, ipl3font, Controllers, Rng};
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
    last_shoot_time: i32,
    health: i32,
    score: i32,
}

impl Player {
    pub fn new() -> Player {
        let player = Player {
            entity: entity::create(),
            last_shoot_time: 0,
            health: 500,
            score: 0,
        };

        movable::add(MovableComponent {
            entity: player.entity.as_entity(),
            pos: START_POS,
            speed: Vec2::zero(),
        });
        char_drawable::add(CharDrawableComponent {
            entity: player.entity.as_entity(),
            color: SHIP_COLOR,
            chr: b'A',
        });

        player
    }

    pub fn pos(&self) -> Vec2 {
        if let Some(movable) = movable::get_component(&self.entity) {
            movable.pos
        } else {
            Vec2::zero()
        }
    }

    pub fn damage(&mut self, damage: i32) {
        self.health = 0.max(self.health - damage);
    }

    pub fn add_score(&mut self, score: i32) {
        self.score += score;
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn health(&self) -> i32 {
        self.health
    }

    pub fn is_dead(&self) -> bool {
        self.health <= 0
    }

    pub fn update(
        &mut self,
        controllers: &Controllers,
        bullet_system: &mut BulletSystem,
        rng: &mut Rng,
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

            if now - self.last_shoot_time > SHIP_SHOOT_DELAY_MS * 1000 {
                if controllers.z() {
                    bullet_system.shoot_bullet(
                        rng,
                        movable.pos,
                        Vec2::new(0.0, movable.speed.y() - 0.65),
                    );
                    self.last_shoot_time = now;
                }
            }
        }
    }
}
