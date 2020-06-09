use zerocopy::LayoutVerified;

pub struct StaticSoundData {
    pub data: &'static [u8],
}

impl StaticSoundData {
    pub fn as_sound_data(&self) -> SoundData {
        let samples = LayoutVerified::<_, [i16]>::new_slice(self.data)
            .unwrap()
            .into_slice();

        SoundData { samples }
    }
}

#[derive(Copy, Clone)]
pub struct SoundData {
    pub samples: &'static [i16],
}
