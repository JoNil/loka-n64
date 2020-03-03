#[derive(Copy, Clone)]
pub struct Texture {
    _width: i32,
    _height: i32,
    _data: &'static [u8],
}

impl Texture {
    pub const fn from_static(width: i32, height: i32, data: &'static [u8]) -> Texture {
        Texture {
            _width: width,
            _height: height,
            _data: data,
        }
    }
}
