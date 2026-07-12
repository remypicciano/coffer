# Coffer

Coffer is a cross-platform desktop interface for protecting local files with a separate unlock key. The application is currently a UI prototype: protection and restoration flows are implemented visually, but production cryptography and file writing are not connected yet.

## Development

Requirements:

- Rust toolchain with edition 2024 support
- Platform dependencies required by `eframe` and `rfd`

Run the application:

```sh
cargo run
```

Verify changes:

```sh
cargo fmt -- --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## macOS release build

```sh
cargo build --release
cp target/release/coffer dist/macos/Coffer.app/Contents/MacOS/coffer
open dist/macos/Coffer.app
```

The local `dist/` directory is intentionally excluded from Git.

## Windows x64 build

An ARM Windows virtual machine may select the ARM64 target automatically. To build for conventional x64 Windows:

```sh
rustup target add x86_64-pc-windows-msvc
cargo build --release --target x86_64-pc-windows-msvc
```

## Project documentation

- [Container format proposal](docs/coffer-format-v1.md)
- [Product roadmap](docs/roadmap.md)
- [Development and repository hygiene](docs/development.md)
