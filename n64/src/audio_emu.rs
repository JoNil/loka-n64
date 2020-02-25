pub const BUFFER_NO_SAMPLES: usize = 2 * 512;

#[inline]
pub(crate) fn init() {}

#[inline]
pub fn write_audio_blocking(buffer: &[i16]) {}

#[inline]
pub fn all_buffers_are_full() -> bool {
    false
}

#[inline]
pub fn update() {}
