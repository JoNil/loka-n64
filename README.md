# loka-n64

```bash
rustup component add rust-src
cargo install cargo-xbuild
```

```bash
cargo +nightly n64 build --ipl3 bootcode.bin --package .
```