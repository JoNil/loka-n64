use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    if env::args().len() < 2 {
        println!("Usage: {} [FILE]", env::args().next().unwrap());
        return Ok(());
    }

    let rom = fs::read(env::args().nth(1).unwrap())?;

    let bootcode = &rom[0x40..(0x40 + 4032)];

    fs::write("bootcode.bin", bootcode)?;

    Ok(())
}
