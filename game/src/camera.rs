use n64::Controllers;
use n64_math::Vec2;

pub struct Camera {
    pub pos: Vec2,
    dpad_pressed_last_frame: bool,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            pos: Vec2::zero(),
            dpad_pressed_last_frame: false,
        }
    }

    pub fn update(&mut self, controllers: &Controllers) {
        if controllers.c_up() {
            self.pos.set_y(self.pos.y() - 10.0);
        }

        if controllers.c_down() {
            self.pos.set_y(self.pos.y() + 10.0);
        }

        if controllers.c_left() {
            self.pos.set_x(self.pos.x() - 10.0);
        }

        if controllers.c_right() {
            self.pos.set_x(self.pos.x() + 10.0);
        }

        let mut dpad_pressed_this_frame = false;

        if controllers.up() {
            if !self.dpad_pressed_last_frame {
                self.pos.set_y(self.pos.y() as i32 as f32 - 1.0);
            }
            dpad_pressed_this_frame = true;
        }

        if controllers.down() {
            if !self.dpad_pressed_last_frame {
                self.pos.set_y(self.pos.y() as i32 as f32 + 1.0);
            }
            dpad_pressed_this_frame = true;
        }

        if controllers.left() {
            if !self.dpad_pressed_last_frame {
                self.pos.set_x(self.pos.x() as i32 as f32 - 1.0);
            }
            dpad_pressed_this_frame = true;
        }

        if controllers.right() {
            if !self.dpad_pressed_last_frame {
                self.pos.set_x(self.pos.x() as i32 as f32 + 1.0);
            }
            dpad_pressed_this_frame = true;
        }

        self.dpad_pressed_last_frame = dpad_pressed_this_frame;
    }
}
