use crate::utils::{write_binary_file_if_changed, write_file_if_changed};
use itertools::Itertools;
use std::{env, ffi::OsStr, fs, path::Path};
use zerocopy::AsBytes;

fn load_wav(path: impl AsRef<Path>) -> Vec<i16> {
    println!("rerun-if-changed={}", path.as_ref().to_string_lossy());

    let reader = hound::WavReader::open(path.as_ref())
        .map_err(|e| format!("Unable to load: {}, {}", path.as_ref().to_string_lossy(), e))
        .unwrap();

    let spec = reader.spec();

    assert!(spec.channels == 2 || spec.channels == 1);
    assert!(spec.sample_rate == 22050);
    assert!(spec.bits_per_sample == 16);
    assert!(spec.sample_format == hound::SampleFormat::Int);

    let mut data = Vec::with_capacity(reader.duration() as usize);

    if spec.channels == 1 {
        for sample in reader.into_samples::<i16>().filter_map(|e| e.ok()) {
            data.push(sample.swap_bytes());
        }
    } else if spec.channels == 2 {
        for (l, r) in reader
            .into_samples::<i16>()
            .filter_map(|e| e.ok())
            .tuple_windows::<(_, _)>()
        {
            data.push((((l as i32 + r as i32) / 2) as i16).swap_bytes());
        }
    }

    data
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

pub(crate) fn parse() {
    let mut sounds = String::new();

    for path in fs::read_dir("sounds")
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|path| path.extension() == Some(OsStr::new("wav")))
    {
        if let Some(name) = path.file_stem().map(|n| n.to_string_lossy()) {
            let out_path = path.canonicalize().unwrap().with_extension("nsnd");
            let wav = load_wav(&path);

            write_binary_file_if_changed(&out_path, wav.as_bytes()).unwrap();

            sounds.push_str(&format!(
                SOUND_TEMPLATE!(),
                name = name.to_uppercase(),
                path = out_path,
            ));
        }
    }

    let sounds = format!(SOUNDS_TEMPLATE!(), sounds = sounds);

    write_file_if_changed(
        env::current_dir().unwrap().join("src").join("sounds.rs"),
        sounds,
    )
    .unwrap();
}
