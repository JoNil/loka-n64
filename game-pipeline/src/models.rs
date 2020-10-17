use crate::utils::write_file_if_changed;
use std::{env, error::Error, ffi::OsStr, fs};

#[rustfmt::skip]
macro_rules! MODEL_TEMPLATE { () => {
r##"pub static {name}: StaticSoundData = StaticSoundData {{ data: include_bytes_align_as!(i16, {path:?}) }};
"##
}; }

#[rustfmt::skip]
macro_rules! MODELS_TEMPLATE { () => {
r##"// This file is generated

#![cfg_attr(rustfmt, rustfmt::skip)]

//use crate::sound::StaticSoundData;
//use n64::include_bytes_align_as;

{models}"##
}; }

pub(crate) fn parse() -> Result<(), Box<dyn Error>> {
    let mut models = String::new();

    for path in fs::read_dir("models")?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|path| path.extension() == Some(OsStr::new("blend")))
    {
        //panic!("{:?}", path);

        /*write_binary_file_if_changed(&out_path, wav.as_bytes())?;

        models.push_str(&format!(
            MODEL_TEMPLATE!(),
            name = name.to_uppercase(),
            path = out_path,
        ));*/
    }

    let models = format!(MODELS_TEMPLATE!(), models = models);

    write_file_if_changed(env::current_dir()?.join("src").join("models.rs"), models)?;

    Ok(())
}
