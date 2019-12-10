use n64_sys::ai;

#[inline]
pub(crate) fn init() {
    ai::init();
}

pub fn write_audio_blocking(buffer: &[i16; 2 * 512]) {
    ai::write_audio_blocking(buffer);
}