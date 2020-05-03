pub const BUFFER_NO_SAMPLES: usize = 2 * 512;

pub struct Audio {}

impl Audio {
    #[inline]
    pub(crate) fn new() -> Self { Self {} }

    #[inline]
    pub fn write_audio_blocking(&mut self, _buffer: &[i16]) {}

    #[inline]
    pub fn all_buffers_are_full(&self) -> bool {
        true
    }

    #[inline]
    pub fn update(&mut self) {
    }
}
