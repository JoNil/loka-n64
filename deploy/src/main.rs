use std::env;
use std::error::Error;
use std::process::{Command, Stdio};

fn main() -> Result<(), Box<dyn Error>> {
    if !env::current_dir()?.ends_with("loka-n64") {
        env::set_current_dir("../")?;
    }

    assert!(Command::new("cargo")
        .args(&[
            "+nightly",
            "n64",
            "build",
            "--ipl3",
            "bootcode.bin",
            "--package",
            "game",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?
        .success());

    assert!(Command::new("tools/usb64.exe")
        .args(&[
            "-rom=target/mips-nintendo64-none/release/game.n64",
            "-start"
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?
        .success());

    Ok(())
}
