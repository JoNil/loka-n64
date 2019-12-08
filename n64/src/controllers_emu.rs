use minifb::Key;
use crate::graphics;

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

    pub fn x(&self) -> i8 {
        let mut res = 0;

        if self.data.contains(&Key::Right) {
            res += 127;
        }

        if self.data.contains(&Key::Left) {
            res -= 127;
        }

        res
    }

    pub fn y(&self) -> i8 {
        let mut res = 0;

        if self.data.contains(&Key::Up) {
            res += 127;
        }

        if self.data.contains(&Key::Down) {
            res -= 127;
        }

        res
    }

    #[inline]
    pub fn a(&self) -> bool {
        self.data.contains(&Key::X)
    }

    #[inline]
    pub fn b(&self) -> bool {
        self.data.contains(&Key::C)
    }

    #[inline]
    pub fn z(&self) -> bool {
        self.data.contains(&Key::Space)
    }
}
