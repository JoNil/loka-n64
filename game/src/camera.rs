use n64::{VideoMode, Controllers};
use n64_math::Vec2;

pub const SPEED: f32 = 16.0/240.0;

pub struct Camera {
    pub pos: Vec2,
    pub speed: Vec2,
    dpad_pressed_last_frame: bool,
    debug_camera: bool,
}

impl Camera {
    pub fn new(start_pos: Vec2) -> Self {
        Self {
            pos: start_pos,
            speed: Vec2::new(0.0, SPEED),
            dpad_pressed_last_frame: false,
            debug_camera: false,
        }
    }

    pub fn update(&mut self, controllers: &Controllers, dt: f32) {
        if !self.debug_camera {
            self.pos.1 -= self.speed.1 * dt;
            // Stop at top.
            if self.pos.1 < 0.0 {
                self.pos.1 = 0.0;
                self.speed.1 = 0.0;
            }
        }

        if controllers.c_up() {
            self.debug_camera = true;
            self.pos.set_y(self.pos.y() - 10.0);
        }

        if controllers.c_down() {
            self.debug_camera = true;
            self.pos.set_y(self.pos.y() + 10.0);
        }

        if controllers.c_left() {
            self.debug_camera = true;
            self.pos.set_x(self.pos.x() - 10.0);
        }

        if controllers.c_right() {
            self.debug_camera = true;
            self.pos.set_x(self.pos.x() + 10.0);
        }

        let mut dpad_pressed_this_frame = false;

        if controllers.up() {
            self.debug_camera = true;
            if !self.dpad_pressed_last_frame {
                self.pos.set_y(self.pos.y() as i32 as f32 - 1.0);
            }
            dpad_pressed_this_frame = true;
        }

        if controllers.down() {
            self.debug_camera = true;
            if !self.dpad_pressed_last_frame {
                self.pos.set_y(self.pos.y() as i32 as f32 + 1.0);
            }
            dpad_pressed_this_frame = true;
        }

        if controllers.left() {
            self.debug_camera = true;
            if !self.dpad_pressed_last_frame {
                self.pos.set_x(self.pos.x() as i32 as f32 - 1.0);
            }
            dpad_pressed_this_frame = true;
        }

        if controllers.right() {
            self.debug_camera = true;
            if !self.dpad_pressed_last_frame {
                self.pos.set_x(self.pos.x() as i32 as f32 + 1.0);
            }
            dpad_pressed_this_frame = true;
        }

        self.dpad_pressed_last_frame = dpad_pressed_this_frame;
    }
}
