# loka-n64

## Build Environment Setup

```bash
rustup install nightly

cargo +nightly install -f --git https://github.com/JoNil/cargo-n64.git --branch master cargo-n64

rustup default nightly
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
cargo +nightly n64 build --ipl3 bootcode.bin -- --package game
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
- https://github.com/PeterLemon/N64/blob/master/RDP/16BPP/Rectangle/FillRectangle/FillRectangle320x240/FillRectangle16BPP320X240.asm
- http://krikzz.com/pub/support/everdrive-64/x-series/dev/
- https://dragonminded.com/n64dev/Reality%20Coprocessor.pdf
- https://github.com/Themaister/parallel-rdp
- https://ultra64.ca/resources/documentation/
- http://ultra64.ca/files/documentation/silicon-graphics/SGI_RDP_Command_Summary.pdf
- http://ultra64.ca/files/documentation/silicon-graphics/SGI_Nintendo_64_RSP_Programmers_Guide.pdf
- http://ultra64.ca/files/documentation/online-manuals/man/pro-man/pro12/index12.6.html
- https://ultra64.ca/files/documentation/nintendo/Nintendo_64_Programming_Manual_NU6-06-0030-001G_HQ.pdf

How to kick off sp tasks
- https://github.com/n64decomp/sm64/blob/master/lib/src/osSpTaskLoadGo.c

Mips
- https://hack64.net/wiki/doku.php?id=r4300
- https://www.kth.se/social/files/563c63c9f276547044e8695f/mips-ref-sheet.pdf