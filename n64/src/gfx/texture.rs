use n64_math::Color;
use zerocopy::LayoutVerified;

#[repr(align(8))]
pub struct TextureAlignment;

#[derive(Copy, Clone)]
pub struct Texture<'a> {
    pub width: i32,
    pub height: i32,
    pub data: &'a [Color],
}

impl<'a> Texture<'a> {
    #[inline]
    pub fn new(width: i32, height: i32, data: &'a [Color]) -> Self {
        Self {
            width,
            height,
            data,
        }
    }
}

pub struct TextureMut<'a> {
    pub width: i32,
    pub height: i32,
    pub data: &'a mut [Color],
}

impl<'a> TextureMut<'a> {
    #[inline]
    pub fn new(width: i32, height: i32, data: &'a mut [Color]) -> Self {
        Self {
            width,
            height,
            data,
        }
    }

    #[inline]
    pub fn into_texture(self) -> Texture<'a> {
        Texture {
            width: self.width,
            height: self.height,
            data: self.data,
        }
    }
}

#[derive(Copy, Clone)]
pub struct StaticTexture {
    pub width: i32,
    pub height: i32,
    // Should be 8 byte aligned
    pub data: &'static [u8],
}

impl StaticTexture {
    #[inline]
    pub const fn from_static(width: i32, height: i32, data: &'static [u8]) -> Self {
        Self {
            width,
            height,
            data,
        }
    }

    #[inline]
    pub fn as_texture(self) -> Texture<'static> {
        let data = LayoutVerified::<_, [Color]>::new_slice_unaligned(self.data)
            .unwrap()
            .into_slice();

        Texture {
            width: self.width,
            height: self.height,
            data,
        }
    }
}
