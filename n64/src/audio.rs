use n64_sys::ai;

pub use n64_sys::ai::BUFFER_NO_SAMPLES;

pub struct Audio {}

impl Audio {
    #[inline]
    pub(crate) fn new() -> Self {
        ai::init();
        Self {}
    }

    #[inline]
    pub fn write_audio_blocking(&mut self, buffer: &[i16]) {
        ai::write_audio_blocking(buffer);
    }

    #[inline]
    pub fn all_buffers_are_full(&self) -> bool {
        ai::all_buffers_are_full()
    }

    #[inline]
    pub fn update(&mut self) {
        ai::submit_audio_data_to_dac();
    }
}
