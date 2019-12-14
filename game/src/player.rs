use crate::bullet_system::BulletSystem;
use n64::{current_time_us, graphics, ipl3font, Controllers, Rng};
use n64_math::{Aabb2, Color, Vec2};

const START_POS: Vec2 = Vec2::new(0.5, 0.8);
const SHIP_COLOR: Color = Color::new(0b10000_00011_00011_1);
const SHIP_SPEED: f32 = 0.35;
const SHIP_SHOOT_DELAY_MS: i32 = 150;
pub const SHIP_SIZE: Vec2 = Vec2::new(
    ipl3font::GLYPH_WIDTH as f32 / graphics::WIDTH as f32,
    ipl3font::GLYPH_HEIGHT as f32 / graphics::HEIGHT as f32,
);

pub struct Player {
    pos: Vec2,
    last_shoot_time: i32,
    health: i32,
    score: i32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: START_POS,
            last_shoot_time: 0,
            health: 500,
            score: 0,
        }
    }

    #[inline]
    pub fn pos(&self) -> Vec2 {
        self.pos
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
        dt: f32,
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

        let speed = SHIP_SPEED * controller_dir;

        {
            let now = current_time_us();

            if now - self.last_shoot_time > SHIP_SHOOT_DELAY_MS * 1000 {
                if controllers.z() {
                    bullet_system.shoot_bullet(rng, self.pos, Vec2::new(0.0, speed.y() - 0.65));
                    self.last_shoot_time = now;
                }
            }
        }

        self.pos += dt * speed;
    }

    pub fn draw(&self) {
        let screen_x = (self.pos.x() * (graphics::WIDTH as f32)) as i32 - ipl3font::GLYPH_WIDTH / 2;
        let screen_y =
            (self.pos.y() * (graphics::HEIGHT as f32)) as i32 + ipl3font::GLYPH_HEIGHT / 2;

        ipl3font::draw_str(screen_x, screen_y, SHIP_COLOR, b"A");
    }
}
