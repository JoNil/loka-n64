use minifb::Key;
use crate::graphics;

pub(crate) fn init() {}

pub struct Controllers {
    data: Vec<Key>,
}

impl Controllers {
    pub fn new() -> Controllers {
        Controllers {
            data: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        self.data = graphics::get_keys();
    }

    pub fn up_pressed(&self) -> bool {
        self.data.contains(&Key::Up)
    }

    pub fn down_pressed(&self) -> bool {
        self.data.contains(&Key::Down)
    }

    pub fn left_pressed(&self) -> bool {
        self.data.contains(&Key::Left)
    }

    pub fn right_pressed(&self) -> bool {
        self.data.contains(&Key::Right)
    }
}
