use n64_math::Vec2;
use n64::{graphics, ipl3font};

const MAX_BULLETS: usize = 512;
const BULLET_COLOR_INIT: u16 = 0b00001_11000_11000_1;

#[derive(Copy, Clone, Default)]
struct Bullet {
    pos: Vec2,
    speed: Vec2,
    color: u16,
}

pub struct BulletSystem {
    bullets: [Bullet; MAX_BULLETS],
    next_free_index: usize,
}

impl BulletSystem {

    pub fn new() -> BulletSystem {
        BulletSystem {
            bullets: [Default::default(); MAX_BULLETS],
            next_free_index: 0
        }
    }

    pub fn shoot_bullet(&mut self, pos: Vec2, speed: Vec2) {

        if self.next_free_index >= MAX_BULLETS {
            return;
        }

        self.bullets[self.next_free_index] = Bullet {
            pos: pos,
            speed: Vec2::new(0.0, speed.y() - 0.75),
            color: BULLET_COLOR_INIT,
        };

        self.next_free_index += 1; 
    }

    pub fn update(&mut self, dt: f32) {
        for bullet in self.bullets[..self.next_free_index].iter_mut() {

            bullet.pos += dt * bullet.speed;
        }
    }

    pub fn draw(&self) {
        for bullet in self.bullets[..self.next_free_index].iter() {

            let screen_x = (bullet.pos.x() * (graphics::WIDTH as f32)) as i32;
            let screen_y = (bullet.pos.y() * (graphics::HEIGHT as f32)) as i32;

            ipl3font::draw_str(screen_x, screen_y, bullet.color, b".");

        }
    }
}