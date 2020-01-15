use std::convert::TryInto;
use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    if env::args().len() < 2 {
        println!("Usage: {} [FILE]", env::args().nth(0).unwrap());
        return Ok(());
    }

    let name = env::args().nth(1).unwrap();

    let rom = fs::read(&name)?;

    let mut res = Vec::new();

    for word in rom.chunks(4) {
        let mut value = u32::from_le_bytes(word.try_into()?);
        value = value.swap_bytes();
        res.extend_from_slice(&value.to_le_bytes());
    }

    fs::write(format!("{}.swaped", &name), res)?;

    Ok(())
}
