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
    done: Vec<i32>,
}

impl SoundMixer {
    pub fn new() -> Self {
        Self {
            playing_sounds: Vec::with_capacity(16),
            done: Vec::with_capacity(16),
        }
    }

    pub fn play_sound(&mut self, sound: SoundData) {
        if self.playing_sounds.len() == 16 {
            return;
        }

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
                        accumulator +=
                            playing_sound.sound.samples[playing_sound.current_sample] as i32;
                    }

                    #[cfg(not(target_vendor = "nintendo64"))]
                    {
                        accumulator += playing_sound.sound.samples[playing_sound.current_sample]
                            .swap_bytes() as i32;
                    }

                    playing_sound.current_sample += 1;
                }
            }

            let accumulator = accumulator.clamp(i16::MIN as i32, i16::MAX as i32) as i16;

            out_sample[0] = accumulator;
            out_sample[1] = accumulator;
        }

        for (i, playing_sound) in self.playing_sounds.iter().enumerate() {
            if playing_sound.done {
                self.done.push(i as i32);
            }
        }

        for done in self.done.iter().rev() {
            self.playing_sounds.swap_remove(*done as usize);
        }

        self.done.clear();
    }
}
