# loka-n64

## Build Environment Setup

```bash
rustup install nightly-2020-05-14

cargo +nightly-2020-05-14 install -f --git https://github.com/rust-console/cargo-n64.git cargo-n64

rustup default nightly-2020-05-14
rustup component add rust-src
rustup default stable
```

## Extract bootcode

Download mario 64

```bash
cargo run --package extract_boot_code -- "roms/Super Mario 64 (U) [!].z64"
```

## Build for N64

```bash
cargo +nightly n64 build --ipl3 bootcode.bin --package game
```

## Run for PC

```bash
cargo run -p game --release
```

## Run on N64 with EverDrive-64 X7

```bash
cargo run
```

## Links

- https://github.com/command-tab/awesome-n64-development
- http://n64dev.50megs.com/n64dox.html
- http://n64dev.org/
- https://github.com/n64decomp/sm64/
- https://github.com/n64decomp/libreultra
- https://www.kth.se/social/files/563c63c9f276547044e8695f/mips-ref-sheet.pdf
- https://github.com/PeterLemon/N64/blob/master/RDP/16BPP/Rectangle/FillRectangle/FillRectangle320x240/FillRectangle16BPP320X240.asm
- http://ultra64.ca/files/documentation/silicon-graphics/SGI_RDP_Command_Summary.pdf
- http://ultra64.ca/files/documentation/silicon-graphics/SGI_Nintendo_64_RSP_Programmers_Guide.pdf
- http://krikzz.com/pub/support/everdrive-64/x-series/dev/
- https://dragonminded.com/n64dev/Reality%20Coprocessor.pdf
- https://github.com/Themaister/parallel-rdp