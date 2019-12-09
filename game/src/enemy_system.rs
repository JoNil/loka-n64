use crate::bullet_system::BulletSystem;
use n64::{graphics, ipl3font, Rng};
use n64_math::{Color, Vec2};

const MAX_ENEMIES: usize = 128;
pub const ENEMY_SIZE: Vec2 = Vec2::new(
    ipl3font::GLYPH_WIDTH as f32 / graphics::WIDTH as f32,
    ipl3font::GLYPH_HEIGHT as f32 / graphics::HEIGHT as f32,
);

#[derive(Copy, Clone, Default)]
pub struct Enemy {
    pos: Vec2,
    color: Color,
    health: i32,
}

impl Enemy {
    #[inline]
    pub fn pos(&self) -> Vec2 {
        self.pos
    }

    pub fn damage(&mut self, damage: i32) {
        self.health = 0.max(self.health - damage);
    }
}

pub struct EnemySystem {
    enemies: [Enemy; MAX_ENEMIES],
    next_free_index: usize,
    last_spawn_time: i32,
}

impl EnemySystem {
    pub fn new() -> EnemySystem {
        EnemySystem {
            enemies: [Default::default(); MAX_ENEMIES],
            next_free_index: 0,
            last_spawn_time: 0,
        }
    }

    pub fn spawn_enemy(&mut self, rng: &mut Rng) {
        if self.next_free_index >= MAX_ENEMIES {
            return;
        }

        self.enemies[self.next_free_index] = Enemy {
            pos: Vec2::new(rng.next_f32(), rng.next_f32() * 0.6),
            color: Color::from_rgb(rng.next_f32(), rng.next_f32(), rng.next_f32()),
            health: 100,
        };

        self.next_free_index += 1;
    }

    pub fn update(&mut self, bullet_system: &mut BulletSystem, rng: &mut Rng) {
        let mut delete_list = [false; MAX_ENEMIES];

        for (i, enemy) in self.enemies_mut().iter_mut().enumerate() {
            if enemy.health <= 0 {
                delete_list[i] = true;
            }
        }

        for (i, delete) in delete_list[..self.next_free_index].iter().enumerate() {
            if *delete {
                if self.next_free_index > 0 {
                    self.enemies.swap(i, self.next_free_index - 1);
                }
                self.next_free_index -= 1;
            }
        }
    }

    pub fn draw(&self) {
        for enemy in self.enemies().iter() {
            let screen_x =
                (enemy.pos.x() * (graphics::WIDTH as f32)) as i32 - ipl3font::GLYPH_WIDTH / 2;
            let screen_y =
                (enemy.pos.y() * (graphics::HEIGHT as f32)) as i32 + ipl3font::GLYPH_HEIGHT / 2;

            ipl3font::draw_str(screen_x, screen_y, enemy.color, b"#");
        }
    }

    #[inline]
    pub fn enemies(&self) -> &[Enemy] {
        &self.enemies[..self.next_free_index]
    }

    #[inline]
    pub fn enemies_mut(&mut self) -> &mut [Enemy] {
        &mut self.enemies[..self.next_free_index]
    }
}
