use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;
    game_pipeline::run(&out_dir)?;
    Ok(())
}
