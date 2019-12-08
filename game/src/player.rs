use crate::bullet_system::BulletSystem;
use n64_math::Vec2;
use n64::{controllers::Controllers, graphics, ipl3font, current_time_us};

const START_POS: Vec2 = Vec2::new(0.5, 0.8);
const SHIP_COLOR: u16 = 0b10000_00011_00011_1;
const SHIP_SPEED: f32 = 0.35;
const SHIP_SHOOT_DELAY_MS: i32 = 750;

pub struct Player {
    pos: Vec2,
    last_shoot_time: i32,
}

impl Player {

    pub fn new() -> Player {
        Player {
            pos: START_POS,
            last_shoot_time: 0,
        }
    }

    pub fn update(&mut self, dt: f32, controllers: &Controllers, bullet_system: &mut BulletSystem) {
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
                    bullet_system.shoot_bullet(self.pos, speed);
                    self.last_shoot_time = now;
                }
            }
        }

        self.pos += dt * speed;
    }

    pub fn draw(&self) {

        let screen_x = (self.pos.x() * (graphics::WIDTH as f32)) as i32 - ipl3font::GLYPH_WIDTH/2;
        let screen_y = (self.pos.y() * (graphics::HEIGHT as f32)) as i32 + ipl3font::GLYPH_HEIGHT/2;

        ipl3font::draw_str(screen_x, screen_y, SHIP_COLOR, b"A");
    }
}