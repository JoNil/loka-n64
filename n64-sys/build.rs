use std::{fs, path::Path, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=rsp");

    let bass = Path::new("../tools/bass/bass.exe").canonicalize().unwrap();
    let bass = bass.to_string_lossy();
    let bass = bass.trim_start_matches("\\\\?\\");

    for file in fs::read_dir("rsp").unwrap().filter_map(|e| e.ok()) {
        let path = file.path();
        let ext = path.extension().map(|s| s.to_string_lossy());

        if let Some(ext) = ext {
            if ext == "asm" {
                let path = path.canonicalize().unwrap();
                let path = path.to_string_lossy();
                let path = path.trim_start_matches("\\\\?\\");

                assert!(Command::new(&bass).arg(path).status().is_ok());
            }
        }
    }
}
