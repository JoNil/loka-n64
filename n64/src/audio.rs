use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use n64_sys::ai;

const BUFFER_COUNT: usize = 4;
const BUFFER_NO_SAMPLES: usize = 2 * 880;

pub struct Audio {
    free_buffers: VecDeque<Box<[i16]>>,
    ready_buffers: VecDeque<Box<[i16]>>,
    playing_buffers: VecDeque<Box<[i16]>>,
}

impl Audio {
    #[inline]
    pub(crate) fn new() -> Self {
        ai::init();

        let mut free_buffers = VecDeque::with_capacity(BUFFER_COUNT);
        let ready_buffers = VecDeque::with_capacity(BUFFER_COUNT);
        let mut playing_buffers = VecDeque::with_capacity(BUFFER_COUNT);

        for _ in 0..(BUFFER_COUNT / 2) {
            let mut buffer = Vec::new();
            buffer.resize_with(BUFFER_NO_SAMPLES, Default::default);
            free_buffers.push_back(buffer.into_boxed_slice());
        }

        for _ in 0..(BUFFER_COUNT / 2) {
            let mut buffer = Vec::new();
            buffer.resize_with(BUFFER_NO_SAMPLES, Default::default);
            playing_buffers.push_back(buffer.into_boxed_slice());
        }

        Self {
            free_buffers,
            ready_buffers,
            playing_buffers,
        }
    }

    #[inline]
    pub fn update(&mut self, mut f: impl FnMut(&mut [i16])) {
        for mut buffer in self.free_buffers.drain(..) {
            f(&mut buffer);
            self.ready_buffers.push_back(buffer);
        }

        while !ai::full() && self.ready_buffers.len() > 0 && self.playing_buffers.len() > 0 {
            self.free_buffers
                .push_back(self.playing_buffers.pop_front().unwrap());

            {
                let next_buffer = self.ready_buffers.pop_front().unwrap();
                ai::submit_audio_data_to_dac(&next_buffer);
                self.playing_buffers.push_back(next_buffer);
            }
        }
    }
}
