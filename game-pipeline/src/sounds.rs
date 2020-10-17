use crate::utils::{write_binary_file_if_changed, write_file_if_changed};
use std::{env, error::Error, ffi::OsStr, fs, path::Path};
use zerocopy::AsBytes;

fn load_wav(path: impl AsRef<Path>) -> Result<Vec<i16>, Box<dyn Error>> {
    println!("rerun-if-changed={}", path.as_ref().to_string_lossy());

    let reader = hound::WavReader::open(path.as_ref())
        .map_err(|e| format!("Unable to load: {}, {}", path.as_ref().to_string_lossy(), e))?;

    let spec = reader.spec();

    assert!(spec.channels == 2 || spec.channels == 1);
    assert!(spec.sample_rate == 22050);
    assert!(spec.bits_per_sample == 16);
    assert!(spec.sample_format == hound::SampleFormat::Int);

    let mut data = Vec::with_capacity(2 * reader.duration() as usize);

    for sample in reader.into_samples::<i16>().filter_map(|e| e.ok()) {
        data.push(sample.swap_bytes());

        if spec.channels == 1 {
            data.push(sample.swap_bytes());
        }
    }

    Ok(data)
}

#[rustfmt::skip]
macro_rules! SOUND_TEMPLATE { () => {
r##"pub static {name}: StaticSoundData = StaticSoundData {{ data: include_bytes_align_as!(i16, {path:?}) }};
"##
}; }

#[rustfmt::skip]
macro_rules! SOUNDS_TEMPLATE { () => {
r##"// This file is generated

#![cfg_attr(rustfmt, rustfmt::skip)]

use crate::sound::StaticSoundData;
use n64::include_bytes_align_as;

{sounds}"##
}; }

pub(crate) fn parse() -> Result<(), Box<dyn Error>> {
    let mut sounds = String::new();

    for path in fs::read_dir("sounds")?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|path| path.extension() == Some(OsStr::new("wav")))
    {
        if let Some(name) = path.file_stem().map(|n| n.to_string_lossy()) {
            let out_path = path.canonicalize()?.with_extension("nsnd");
            let wav = load_wav(&path)?;

            write_binary_file_if_changed(&out_path, wav.as_bytes())?;

            sounds.push_str(&format!(
                SOUND_TEMPLATE!(),
                name = name.to_uppercase(),
                path = out_path,
            ));
        }
    }

    let sounds = format!(SOUNDS_TEMPLATE!(), sounds = sounds);

    write_file_if_changed(env::current_dir()?.join("src").join("sounds.rs"), sounds)?;

    Ok(())
}
