# loka-n64

```bash
rustup component add rust-src
cargo install cargo-xbuild
```

Build https://github.com/JoNil/cargo-n64 and place cargo-n64.exe in .cargo/bin

```bash
cargo +nightly n64 build --ipl3 bootcode.bin --package loka-n64
```