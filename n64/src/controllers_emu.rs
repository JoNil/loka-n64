use crate::graphics::Graphics;
use std::collections::HashSet;
use winit::event::VirtualKeyCode;

pub struct Controllers {
    data: HashSet<VirtualKeyCode>,
}

impl Controllers {
    #[inline]
    pub fn new() -> Controllers {
        Controllers {
            data: HashSet::new(),
        }
    }

    #[inline]
    pub fn update(&mut self, graphics: &Graphics) {
        self.data = graphics.keys_down.clone();
    }

    #[inline]
    pub fn x(&self) -> i8 {
        let mut res = 0;

        if self.data.contains(&VirtualKeyCode::Right) {
            res += 127;
        }

        if self.data.contains(&VirtualKeyCode::Left) {
            res -= 127;
        }

        res
    }

    #[inline]
    pub fn y(&self) -> i8 {
        let mut res = 0;

        if self.data.contains(&VirtualKeyCode::Up) {
            res += 127;
        }

        if self.data.contains(&VirtualKeyCode::Down) {
            res -= 127;
        }

        res
    }

    #[inline]
    pub fn a(&self) -> bool {
        self.data.contains(&VirtualKeyCode::X)
    }

    #[inline]
    pub fn b(&self) -> bool {
        self.data.contains(&VirtualKeyCode::C)
    }

    #[inline]
    pub fn z(&self) -> bool {
        self.data.contains(&VirtualKeyCode::Space)
    }

    #[inline]
    pub fn start(&self) -> bool {
        self.data.contains(&VirtualKeyCode::Return)
    }

    #[inline]
    pub fn up(&self) -> bool {
        self.data.contains(&VirtualKeyCode::W)
    }

    #[inline]
    pub fn down(&self) -> bool {
        self.data.contains(&VirtualKeyCode::S)
    }

    #[inline]
    pub fn left(&self) -> bool {
        self.data.contains(&VirtualKeyCode::A)
    }

    #[inline]
    pub fn right(&self) -> bool {
        self.data.contains(&VirtualKeyCode::D)
    }

    #[inline]
    pub fn l(&self) -> bool {
        self.data.contains(&VirtualKeyCode::Q)
    }

    #[inline]
    pub fn r(&self) -> bool {
        self.data.contains(&VirtualKeyCode::E)
    }

    #[inline]
    pub fn c_up(&self) -> bool {
        self.data.contains(&VirtualKeyCode::I)
    }

    #[inline]
    pub fn c_down(&self) -> bool {
        self.data.contains(&VirtualKeyCode::K)
    }

    #[inline]
    pub fn c_left(&self) -> bool {
        self.data.contains(&VirtualKeyCode::J)
    }

    #[inline]
    pub fn c_right(&self) -> bool {
        self.data.contains(&VirtualKeyCode::L)
    }
}
