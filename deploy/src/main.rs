use crate::profiler::N64Profiler;
use n64_types::{ProfilerMessageBuffer, MESSAGE_MAGIC_PRINT, MESSAGE_MAGIC_PROFILER};
use serialport::SerialPort;
use std::{
    collections::HashMap,
    env,
    error::Error,
    fs,
    io::{self, Write},
    mem::size_of,
    process::{self, Command},
    time::Duration,
};
use zerocopy::LayoutVerified;

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

fn fetch_scope_names() -> HashMap<i16, String> {
    let file = fs::read_to_string("scope_names.txt").unwrap();

    let mut res = HashMap::new();

    for line in file.lines() {
        let mut parts = line.split(';');
        let id = parts.next().unwrap().trim().parse::<i16>().unwrap();
        let name = parts.next().unwrap().trim();

        res.insert(id, name.to_string());
    }

    res
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

    let scope_names = fetch_scope_names();

    let mut ed = find_everdrive();

    println!("Found EverDrive");

    let rom = fs::read("target/mips-nintendo64-none/release/game.n64").unwrap();

    {
        write_cmd(&mut *ed, b'W', 0x10000000, rom.len() as u32, 0);
        ed.write_all(&rom).unwrap();
        write_cmd(&mut *ed, b's', 0, 0, 0);
    }

    let mut profiler = N64Profiler::default();

    loop {
        let mut buf = [0; 32];

        assert_eq!(ed.read(&mut buf[0..1]).unwrap(), 1);

        if buf[0] == MESSAGE_MAGIC_PRINT {
            assert_eq!(ed.read(&mut buf[1..32]).unwrap(), 31);
            for b in &buf[1..32] {
                print!("{}", *b as char);
                io::stdout().flush().ok();
            }
        }
        if buf[0] == MESSAGE_MAGIC_PROFILER {
            assert_eq!(
                ed.read(&mut buf[1..size_of::<ProfilerMessageBuffer>()])
                    .unwrap(),
                size_of::<ProfilerMessageBuffer>() - 1
            );
            let profiler_message = LayoutVerified::<&[u8], ProfilerMessageBuffer>::new_unaligned(
                &buf[..size_of::<ProfilerMessageBuffer>()],
            )
            .unwrap();
            let profiler_message = profiler_message.into_ref();

            let index = i16::from_be(profiler_message.index);
            let count = i16::from_be(profiler_message.count);

            let scope = profiler_message.get_scope_from_be();

            if index == 0 {
                puffin::GlobalProfiler::lock().new_frame();
            }

            profiler.submit_scope(scope, &scope_names);

            if index == count - 1 {
                profiler.flush_frame();
            }
        }
    }
}
