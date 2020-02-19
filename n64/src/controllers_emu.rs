use crate::graphics;
use winit::event::VirtualKeyCode;

pub struct Controllers {
    data: Vec<VirtualKeyCode>,
}

impl Controllers {
    #[inline]
    pub fn new() -> Controllers {
        Controllers {
            data: Vec::new(),
        }
    }

    #[inline]
    pub fn update(&mut self) {
        self.data = graphics::get_keys();
    }

    #[inline]
    pub fn x(&self) -> i8 {
        let mut res = 0;

        if self.data.contains(&VirtualKeyCode ::Right) {
            res += 127;
        }

        if self.data.contains(&VirtualKeyCode ::Left) {
            res -= 127;
        }

        res
    }

    #[inline]
    pub fn y(&self) -> i8 {
        let mut res = 0;

        if self.data.contains(&VirtualKeyCode ::Up) {
            res += 127;
        }

        if self.data.contains(&VirtualKeyCode ::Down) {
            res -= 127;
        }

        res
    }

    #[inline]
    pub fn a(&self) -> bool {
        self.data.contains(&VirtualKeyCode ::X)
    }

    #[inline]
    pub fn b(&self) -> bool {
        self.data.contains(&VirtualKeyCode ::C)
    }

    #[inline]
    pub fn z(&self) -> bool {
        self.data.contains(&VirtualKeyCode ::Space)
    }
}
