use n64::{graphics, ipl3font, Rng};
use n64_math::{Aabb2, Color, Vec2};

const MAX_BULLETS: usize = 512;
const BULLET_SIZE: Vec2 = Vec2::new(2.0 / 320.0, 2.0 / 320.0);

#[derive(Copy, Clone, Default)]
struct Bullet {
    pos: Vec2,
    speed: Vec2,
    color: Color,
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
            screen_bb: Aabb2::new(Vec2::zero(), Vec2::new(1.0, 1.0)),
        }
    }

    pub fn active_bullets(&self) -> usize {
        self.next_free_index
    }

    pub fn shoot_bullet(&mut self, rng: &mut Rng, pos: Vec2, speed: Vec2) {
        if self.next_free_index >= MAX_BULLETS {
            return;
        }

        self.bullets[self.next_free_index] = Bullet {
            pos: pos,
            speed: Vec2::new(0.0, speed.y() - 0.65),
            color: Color::from_rgb(rng.next_f32(), rng.next_f32(), rng.next_f32()),
        };

        self.next_free_index += 1;
    }

    pub fn update(&mut self, dt: f32) {
        let mut delete_list = [false; MAX_BULLETS];

        for (i, bullet) in self.bullets[..self.next_free_index].iter_mut().enumerate() {
            bullet.pos += dt * bullet.speed;

            let bullet_bb = Aabb2::new_center_size(bullet.pos, BULLET_SIZE);

            if !bullet_bb.collides(&self.screen_bb) {
                delete_list[i] = true;
            }
        }

        for (i, delete) in delete_list[..self.next_free_index].iter().enumerate() {
            if *delete {
                if self.next_free_index > 0 {
                    self.bullets.swap(i, self.next_free_index - 1);
                }
                self.next_free_index -= 1;
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
