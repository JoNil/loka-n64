use n64_sys::ai;

#[inline]
pub(crate) fn init() {
    ai::init();
}

pub fn write_audio_blocking(buffer: &[i16]) -> i32 {
    ai::write_audio_blocking(buffer)
}

pub fn all_buffers_are_full() -> bool {
    ai::all_buffers_are_full()
}

pub fn update() {
    ai::submit_buffers_to_dma()
}