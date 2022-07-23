use std::{env, path::Path};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    game_pipeline::run(Path::new(&out_dir));
}
