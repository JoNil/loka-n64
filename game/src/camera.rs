use n64::Controllers;
use n64_math::Vec2;

pub struct Camera {
    pub pos: Vec2,
}

impl Camera {
    pub fn new() -> Self {
        Self { pos: Vec2::zero() }
    }

    pub fn update(&mut self, controllers: &Controllers) {
        if controllers.c_up() {
            self.pos.set_y(self.pos.y() + 10.0);
        }

        if controllers.c_down() {
            self.pos.set_y(self.pos.y() - 10.0);
        }

        if controllers.c_left() {
            self.pos.set_x(self.pos.x() + 10.0);
        }

        if controllers.c_right() {
            self.pos.set_x(self.pos.x() - 10.0);
        }
    }
}
