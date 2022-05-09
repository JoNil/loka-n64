use n64_types::{ScopeData, MESSAGE_MAGIC_PRINT, MESSAGE_MAGIC_PROFILER};
use serialport::SerialPort;
use std::{
    env,
    error::Error,
    fs,
    io::{self, Write},
    mem::size_of,
    process::{self, Command},
    time::Duration,
};

mod profiler;

fn write_cmd(port: &mut dyn SerialPort, cmd: u8, addr: u32, len: u32, arg: u32) {
    let len = len / 512;

    let data = [
        b'c',
        b'm',
        b'd',
        cmd,
        (addr >> 24) as u8,
        (addr >> 16) as u8,
        (addr >> 8) as u8,
        addr as u8,
        (len >> 24) as u8,
        (len >> 16) as u8,
        (len >> 8) as u8,
        len as u8,
        (arg >> 24) as u8,
        (arg >> 16) as u8,
        (arg >> 8) as u8,
        arg as u8,
    ];

    port.write_all(&data).unwrap();
}

fn find_everdrive() -> Box<dyn SerialPort> {
    let ports = serialport::available_ports().unwrap();

    for port in ports {
        let mut port = serialport::new(port.port_name, 9600)
            .timeout(Duration::from_millis(1000))
            .open()
            .expect("Failed to open port");

        write_cmd(&mut *port, b't', 0, 0, 0);

        let mut buf = [0; 16];
        let len = port.read(&mut buf).unwrap();

        let data = &buf[..len];

        if &data[..4] == b"cmdr" {
            return port;
        }
    }

    println!("Could not find everdrive");
    process::exit(1);
}

fn main() -> Result<(), Box<dyn Error>> {
    let _puffin_server =
        puffin_http::Server::new(&format!("0.0.0.0:{}", puffin_http::DEFAULT_PORT)).ok();

    puffin::set_scopes_on(true);

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
            "--",
            "--package",
            "game",
        ])
        .status()?
        .success());

    let mut ed = find_everdrive();

    println!("Found EverDrive");

    let rom = fs::read("target/mips-nintendo64-none/release/game.n64").unwrap();

    {
        write_cmd(&mut *ed, b'W', 0x10000000, rom.len() as u32, 0);
        ed.write_all(&rom).unwrap();
        write_cmd(&mut *ed, b's', 0, 0, 0);
    }

    loop {
        let mut buf = [0; 1];

        assert_eq!(ed.read(&mut buf).unwrap(), 1);

        if buf[0] == MESSAGE_MAGIC_PRINT {
            for _ in 0..31 {
                assert_eq!(ed.read(&mut buf).unwrap(), 1);
                print!("{}", buf[0] as char);
                //println!("{:?}: {}", buf[0] as char, buf[0]);
                io::stdout().flush().ok();
            }
        }
        if buf[0] == MESSAGE_MAGIC_PROFILER {
            for _ in 0..17 {
                assert_eq!(ed.read(&mut buf).unwrap(), 1);
                //print!("{}", buf[0] as char);
                //println!("{:?}: {}", buf[0] as char, buf[0]);
                //io::stdout().flush().ok();
            }
        }
        //println!("{:?}: {}", buf[0] as char, buf[0]);
        //io::stdout().flush().ok();
        /*if buf[0] == MESSAGE_MAGIC_PRINT {
            let mut size_buf = [0; 2];
            assert_eq!(ed.read(&mut size_buf).unwrap(), 2);

            let scope_count = i16::from_be_bytes(size_buf);

            dbg!(scope_count);

            let mut scopes = Vec::new();
            scopes.resize(scope_count as usize * size_of::<ScopeData>(), 0);

            for i in 0..scope_count {
                assert_eq!(
                    ed.read(
                        &mut scopes[(i as usize * size_of::<ScopeData>())
                            ..((i + 1) as usize * size_of::<ScopeData>())]
                    )
                    .unwrap(),
                    size_of::<ScopeData>()
                );
            }
        }*/
    }
}
