#[derive(Copy, Clone)]
pub enum VideoMode {
    Ntsc320x240,
    Pal640x480,
}

impl VideoMode {
    #[inline]
    pub fn width(self) -> i32 {
        match self {
            VideoMode::Ntsc320x240 => 320,
            VideoMode::Pal640x480 => 640,
        }
    }

    #[inline]
    pub fn height(self) -> i32 {
        match self {
            VideoMode::Ntsc320x240 => 240,
            VideoMode::Pal640x480 => 480,
        }
    }

    #[inline]
    pub fn size(self) -> i32 {
        2 * self.width() * self.height()
    }
}