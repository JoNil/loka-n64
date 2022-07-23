use std::path::Path;

pub mod image;
pub mod maps;
pub mod models;
pub mod sounds;
pub mod textures;
pub mod utils;

pub fn run(out_dir: &Path) {
    textures::parse();
    maps::parse(out_dir);
    sounds::parse();
    models::parse();
}
