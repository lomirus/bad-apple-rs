## Preview

Watch in Bilibili: [[東方] 用 Rust 的方式在 Windows Terminal 打开 Bad Apple!!](https://www.bilibili.com/video/BV1mo4y117Nb/)

## Download

Get latest release [here](https://github.com/lomirus/bad-apple-rs/releases).

|Asset|Note|
|---|---|
|[bad-apple-rs-windows.exe](https://github.com/lomirus/bad-apple-rs/releases/download/v1.3.0/bad-apple-rs-windows.exe)|x86_64-pc-windows-msvc|
|[bad-apple-rs-linux](https://github.com/lomirus/bad-apple-rs/releases/download/v1.3.0/bad-apple-rs-linux)|x86_64-unknown-linux-gnu|
|[Source Code (Zip)](https://github.com/lomirus/bad-apple-rs/archive/refs/tags/v1.3.0.zip)||
|[Source Code (tar.gz)](https://github.com/lomirus/bad-apple-rs/archive/refs/tags/v1.3.0.tar.gz)||

## Build From Source

```bash
git clone git@github.com:lomirus/bad-apple-rs.git
cd bad-apple-rs
# generate `data.rs` from `source.txt`
deno run --allow-read --allow-write preload.ts
# compile the project in to executable file in release mode
cargo build --release
```

## Video Info

- Width : 128 Chars
- Height : 48 Chars
- Frame Rate : 24 FPS
