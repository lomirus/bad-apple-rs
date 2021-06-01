## Preview

Watch in Bilibili: [[東方] 用 Rust 的方式在 Windows Terminal 打开 Bad Apple!!](https://www.bilibili.com/video/BV1mo4y117Nb/)

## Download

Get latest release [here](https://github.com/lomirus/bad-apple-rs/releases).

|Asset|Note|
|---|---|
|[bad-apple-rs.exe](https://github.com/lomirus/bad-apple-rs/releases/download/v1.0.0/bad-apple-rs.exe)|also runs on linux|
|[Source Code (Zip)](https://github.com/lomirus/bad-apple-rs/archive/refs/tags/v1.0.0.zip)||
|[Source Code (tar.gz)](https://github.com/lomirus/bad-apple-rs/archive/refs/tags/v1.0.0.tar.gz)||

## Build From Source

```bash
git clone git@github.com:lomirus/bad-apple-rs.git
cd bad-apple-rs
# generate `data.rs` from `source.txt`
deno run --allow-read --allow-write preload.ts
# compile the project in to executable file in release mode
cargo build --release
```

## Screen Requirements

- WIDHT >= 101 Chars
- HEIGHT >= 44 Chars

Succeed in Windows Terminal full screen mode with default configurations in Windows 10, 1920x1080 Resolution, 125% Scale.