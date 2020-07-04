use n64::{Controllers, VideoMode};
use n64_math::Vec2;

pub const SPEED: f32 = 16.0 / 240.0;

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

    pub fn update(&mut self, controllers: &Controllers, dt: f32, video_mode: &VideoMode) {
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
            self.pos.1 -= 10.0 / video_mode.height() as f32;
        }

        if controllers.c_down() {
            self.debug_camera = true;
            self.pos.1 += 10.0 / video_mode.height() as f32;
        }

        if controllers.c_left() {
            self.debug_camera = true;
            self.pos.0 -= 10.0 / video_mode.width() as f32;
        }

        if controllers.c_right() {
            self.debug_camera = true;
            self.pos.0 += 10.0 / video_mode.width() as f32;
        }

        let mut dpad_pressed_this_frame = false;

        if controllers.up() {
            self.debug_camera = true;
            if !self.dpad_pressed_last_frame {
                self.pos.1 -= 1.0 / video_mode.height() as f32;
            }
            dpad_pressed_this_frame = true;
        }

        if controllers.down() {
            self.debug_camera = true;
            if !self.dpad_pressed_last_frame {
                self.pos.1 += 1.0 / video_mode.height() as f32;
            }
            dpad_pressed_this_frame = true;
        }

        if controllers.left() {
            self.debug_camera = true;
            if !self.dpad_pressed_last_frame {
                self.pos.0 -= 1.0 / video_mode.width() as f32;
            }
            dpad_pressed_this_frame = true;
        }

        if controllers.right() {
            self.debug_camera = true;
            if !self.dpad_pressed_last_frame {
                self.pos.0 += 1.0 / video_mode.width() as f32;
            }
            dpad_pressed_this_frame = true;
        }

        self.dpad_pressed_last_frame = dpad_pressed_this_frame;
    }
}
