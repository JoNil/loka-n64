use std::error::Error;

pub mod image;
pub mod maps;
pub mod models;
pub mod sounds;
pub mod textures;
pub mod utils;

pub fn run(out_dir: &str) -> Result<(), Box<dyn Error>> {
    textures::parse()?;
    maps::parse(out_dir)?;
    sounds::parse()?;
    models::parse()?;

    Ok(())
}
