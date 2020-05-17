use n64_sys::ai;

pub struct Audio {}

impl Audio {
    #[inline]
    pub(crate) fn new() -> Self {
        ai::init();
        Self {}
    }

    #[inline]
    pub fn update(&mut self, mut f: impl FnMut(&mut [i16])) {

        while !ai::all_buffers_are_full() {
            write_audio_blocking(f);
        }

        ai::submit_audio_data_to_dac();
    }
}
