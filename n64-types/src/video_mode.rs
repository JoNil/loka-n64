#[derive(Copy, Clone)]
pub enum VideoMode {
    Ntsc { width: i32, height: i32 },
    Pal { width: i32, height: i32 },
}

impl VideoMode {
    #[inline]
    pub fn width(self) -> i32 {
        match self {
            VideoMode::Ntsc { width, .. } => width,
            VideoMode::Pal { width, .. } => width,
        }
    }

    #[inline]
    pub fn height(self) -> i32 {
        match self {
            VideoMode::Ntsc { height, .. } => height,
            VideoMode::Pal { height, .. } => height,
        }
    }

    #[inline]
    pub fn size(self) -> i32 {
        2 * self.width() * self.height()
    }
}
