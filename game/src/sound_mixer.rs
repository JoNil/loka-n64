use crate::sound::SoundData;
use alloc::vec::Vec;

#[derive(Copy, Clone)]
struct PlayingSound {
    sound: SoundData,
    current_sample: usize,
    done: bool,
}

pub struct SoundMixer {
    playing_sounds: Vec<PlayingSound>,
}

impl SoundMixer {
    pub fn new() -> Self {
        Self {
            playing_sounds: Vec::with_capacity(16),
        }
    }

    pub fn play_sound(&mut self, sound: SoundData) {
        self.playing_sounds.push(PlayingSound {
            sound,
            current_sample: 0,
            done: false,
        });
    }

    pub fn mix(&mut self, buffer: &mut [i16]) {
        for out_sample in buffer.chunks_exact_mut(2) {
            let mut accumulator: i32 = 0;

            for playing_sound in self.playing_sounds.iter_mut() {
                if playing_sound.current_sample >= playing_sound.sound.samples.len() {
                    playing_sound.done = true;
                } else {
                    #[cfg(target_vendor = "nintendo64")]
                    {
                        accumulator = accumulator.saturating_add(
                            playing_sound.sound.samples[playing_sound.current_sample] as i32,
                        );
                    }

                    #[cfg(not(target_vendor = "nintendo64"))]
                    {
                        accumulator = accumulator.saturating_add(
                            playing_sound.sound.samples[playing_sound.current_sample].swap_bytes()
                                as i32,
                        );
                    }

                    playing_sound.current_sample += 1;
                }
            }

            let accumulator = accumulator.min(i16::MAX as i32).max(i16::MIN as i32) as i16;

            out_sample[0] = accumulator;
            out_sample[1] = accumulator;
        }

        self.playing_sounds = self
            .playing_sounds
            .iter()
            .copied()
            .filter(|s| !s.done)
            .collect();
    }
}
