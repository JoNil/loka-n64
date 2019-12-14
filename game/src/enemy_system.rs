use crate::bullet_system::BulletSystem;
use crate::Player;
use n64::{current_time_us, graphics, ipl3font, Rng};
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
    shoot_speed: i32,
    last_shoot_time: i32,
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
}

impl EnemySystem {
    pub fn new() -> EnemySystem {
        EnemySystem {
            enemies: [Default::default(); MAX_ENEMIES],
            next_free_index: 0,
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
            shoot_speed: 500 + (rng.next_f32() * 200.0) as i32,
            last_shoot_time: 0,

        };

        self.next_free_index += 1;
    }

    pub fn update(&mut self, bullet_system: &mut BulletSystem, player: &mut Player, rng: &mut Rng) {
        let mut delete_list = [false; MAX_ENEMIES];

        let now = current_time_us();

        for (i, enemy) in self.enemies_mut().iter_mut().enumerate() {
            if enemy.health <= 0 {
                player.add_score(1000);
                delete_list[i] = true;
            }

            if now - enemy.last_shoot_time > enemy.shoot_speed * 1000 {
                bullet_system.shoot_bullet_enemy(rng, enemy.pos, Vec2::new(0.0, 0.65));
                enemy.last_shoot_time = now;
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
