use n64_sys::ai;

const BUFFER_COUNT: usize = 4;
const BUFFER_NO_SAMPLES: usize = 2 * 512;

pub struct Audio {
    buffers: [[i16; BUFFER_NO_SAMPLES]; BUFFER_COUNT],
    now_playing: usize,
    now_writing: usize,
    buffers_full_bitmask: usize,
}

impl Audio {
    #[inline]
    pub(crate) fn new() -> Self {
        ai::init();
        Self {
            buffers: [[0; BUFFER_NO_SAMPLES]; BUFFER_COUNT],
            now_playing: 0,
            now_writing: 0,
            buffers_full_bitmask: 0,
        }
    }

    #[inline]
    fn next_to_write(&self) -> usize {
        (self.now_writing + 1) % BUFFER_COUNT
    }

    #[inline]
    fn next_playing(&self) -> usize {
        (self.now_playing + 1) % BUFFER_COUNT
    }

    #[inline]
    fn all_buffers_are_full(&self) -> bool {
        self.buffers_full_bitmask & (1 << self.next_to_write()) > 0
    }

    #[inline]
    pub fn update(&mut self, mut f: impl FnMut(&mut [i16])) {
        while !self.all_buffers_are_full() {
            let next = self.next_to_write();
            assert!(self.buffers_full_bitmask & (1 << next) == 0);

            self.buffers_full_bitmask |= 1 << next;
            self.now_writing = next;

            f(&mut self.buffers[next]);
        }

        while !ai::full() {

            {
                let next = self.next_playing();
                if self.buffers_full_bitmask & (1 << next) == 0 {
                    break;
                }
                
                self.buffers_full_bitmask &= !(1 << next);
                self.now_playing = next;
            }

            {
                let next = self.next_playing();
                if self.buffers_full_bitmask & (1 << next) == 0 {
                    break;
                }

                unsafe { ai::submit_audio_data_to_dac(&self.buffers[next]) };
            }
        }
    }
}
