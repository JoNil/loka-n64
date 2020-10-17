use zerocopy::LayoutVerified;

pub struct StaticModelData {
    
}

/*impl StaticSoundData {
    pub fn as_sound_data(&self) -> SoundData {
        let samples = LayoutVerified::<_, [i16]>::new_slice(self.data)
            .unwrap()
            .into_slice();

        SoundData { samples }
    }
}*/

#[derive(Copy, Clone)]
pub struct ModelData {
    
}
