use n64::{graphics, ipl3font};
use n64_math::{Aabb2, Vec2};

const MAX_BULLETS: usize = 512;
const BULLET_SIZE: Vec2 = Vec2::new(2.0 / 320.0, 2.0 / 320.0);
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
    screen_bb: Aabb2,
}

impl BulletSystem {
    pub fn new() -> BulletSystem {
        BulletSystem {
            bullets: [Default::default(); MAX_BULLETS],
            next_free_index: 0,
            screen_bb: Aabb2::new(Vec2::zero(), Vec2::new(1.0, 1.0))
        }
    }

    pub fn shoot_bullet(&mut self, pos: Vec2, speed: Vec2) {
        if self.next_free_index >= MAX_BULLETS {
            return;
        }

        self.bullets[self.next_free_index] = Bullet {
            pos: pos,
            speed: Vec2::new(0.0, speed.y() - 0.65),
            color: BULLET_COLOR_INIT,
            
        };

        self.next_free_index += 1;
    }

    pub fn update(&mut self, dt: f32) {
        for bullet in self.bullets[..self.next_free_index].iter_mut() {
            bullet.pos += dt * bullet.speed;

            let bullet_bb = Aabb2::new_center_size(bullet.pos, BULLET_SIZE);

            if !bullet_bb.collides(&self.screen_bb) {
                println!("OUTSIDE SCREEN");
            }
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
