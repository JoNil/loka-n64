# loka-n64

## Build Environment Setup

```bash
cargo install cargo-xbuild

rustup default nightly
rustup component add rust-src

git clone https://github.com/JoNil/cargo-n64
cd cargo-n64/cargo-n64
cargo build --release

rustup default stable
```

Place cargo-n64.exe in .cargo/bin

## Extract bootcode

```bash
cargo run --bin extract_boot_code -- roms/mario64.z64
```

## Build

```bash
cargo +nightly n64 build --ipl3 bootcode.bin --package game
```

## Links

http://n64dev.org/