#[derive(Copy, Clone)]
pub struct Texture {
    pub width: i32,
    pub height: i32,
    pub data: &'static [u8],
}

impl Texture {
    pub const fn from_static(width: i32, height: i32, data: &'static [u8]) -> Texture {
        Texture {
            width: width,
            height: height,
            data: data,
        }
    }
}
