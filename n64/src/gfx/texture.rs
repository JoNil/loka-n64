#[derive(Copy, Clone)]
pub struct Texture {
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) data: &'static [u8],
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