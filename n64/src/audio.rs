use n64_sys::ai;

pub use n64_sys::ai::BUFFER_NO_SAMPLES;

pub(crate) fn init() {
    ai::init();
}

pub fn write_audio_blocking(buffer: &[i16]) {
    ai::write_audio_blocking(buffer);
}

pub fn all_buffers_are_full() -> bool {
    ai::all_buffers_are_full()
}

pub fn update() {
    ai::submit_audio_data_to_dac();
}