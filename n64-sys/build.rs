use std::{fs, io::BufRead, path::Path, process::Command};

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

                let output = Command::new(bass).arg(path).output().unwrap();

                let mut has_error = false;

                eprintln!("Bass output\n");

                for line in output.stderr.lines().filter_map(|s| s.ok()) {
                    if line.contains("error") {
                        has_error = true;
                    }
                    eprintln!("{line}");
                }

                eprintln!("\nEnd of Bass output");

                if has_error {
                    panic!("Bass Failed");
                }

                if path.ends_with("rsp.asm") {
                    let path_with_txt = Path::new(path).with_extension("txt");
                    let path_with_bin = Path::new(path).with_extension("bin");
                    let bin = fs::read(path_with_bin).unwrap();
                    let mut code = Vec::new();

                    for ins in bin.chunks(4) {
                        let ins = u32::from_be_bytes(ins.try_into().unwrap());
                        code.push(ins);
                    }

                    let mips = mipsasm_rsp::Mipsasm::new();
                    let code = mips.disassemble(&code);

                    let code = code.join("\n");
                    fs::write(path_with_txt, code).unwrap();
                }
            }
        }
    }
}
