use crate::{image::load_png, utils::write_binary_file_if_changed, utils::write_file_if_changed};
use std::{env, error::Error, ffi::OsStr, fs};

#[rustfmt::skip]
macro_rules! TEXTURE_TEMPLATE { () => {
r##"pub static {name}: StaticTexture = StaticTexture::from_static({width}, {height}, include_bytes_align_as!(TextureAlignment, {path:?}));
"##
}; }

#[rustfmt::skip]
macro_rules! TEXTURES_TEMPLATE { () => {
r##"// This file is generated

#![cfg_attr(rustfmt, rustfmt::skip)]

use n64::gfx::{{StaticTexture, TextureAlignment}};
use n64::include_bytes_align_as;

{textures}"##
}; }

pub(crate) fn parse() -> Result<(), Box<dyn Error>> {
    let mut textures = String::new();

    for path in fs::read_dir("textures")?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|path| path.extension() == Some(OsStr::new("png")))
    {
        if let Some(name) = path.file_stem().map(|n| n.to_string_lossy()) {
            let out_path = path.canonicalize()?.with_extension("ntex");
            let image = load_png(path.as_path(), false, None)?;

            write_binary_file_if_changed(&out_path, &image.data)?;

            textures.push_str(&format!(
                TEXTURE_TEMPLATE!(),
                name = name.to_uppercase(),
                width = image.width,
                height = image.height,
                path = out_path
            ));
        }
    }

    let textures = format!(TEXTURES_TEMPLATE!(), textures = textures);

    write_file_if_changed(
        env::current_dir()?.join("src").join("textures.rs"),
        textures,
    )?;

    Ok(())
}
